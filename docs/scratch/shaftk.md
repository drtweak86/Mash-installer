# 🌾 SHAFT K: The Harvest Engine
> **Strategic Mining Plan**
> *"A mine that cannot clean its tunnels will choke on its own ore. The harvest runs after the forge is swept."* — Bard 🍺

## 📜 Project Summary

Shaft K integrates the **MASH Wallpaper Harvest** (`wallpaper_downloader_final.py`, 8 categories, 5999 wallpapers) into the installer, fixes the critical deployment bug that prevents the downloader from being written to disk, and adds the missing **Clean and Polish phases** to the phase registry (improvement 2). The wallpaper harvest is the capstone of the retro-futuristic environment — it runs last, after the system is clean and polished.

---

## 🗂️ Incoming Files Being Integrated

| Incoming File | What It Is | Deploy Target |
|---|---|---|
| `docs/incoming-files/wallpaper_downloader_final.py` | 8-category wallpaper harvester, final edition | `resources/themes/retro-bbc/wallpaper_downloader_final.py` |
| `docs/incoming-files/wallpaper_downloader_README.md` | Usage and integration documentation | `docs/incoming-files/wallpaper_downloader_README.md` (stays, reference only) |

---

## 🛠️ Technical Plan

---

### 1. Promote the Final Wallpaper Script into Resources

#### Why This Needs to Change

**File:** `resources/themes/retro-bbc/wallpaper_downloader_final.py`

The existing file in `resources/themes/retro-bbc/` is a prior version. The incoming `docs/incoming-files/wallpaper_downloader_final.py` is the **final 8-category edition** that merges Claude's robust downloader architecture with Bard's 2 additional categories (judge_dredd, star_wars) for 5999 total wallpapers.

#### 1.1 — Replace the Resource Script

**Exact change:** Replace the entire contents of `resources/themes/retro-bbc/wallpaper_downloader_final.py` with the content of `docs/incoming-files/wallpaper_downloader_final.py`.

The final edition adds:
- `judge_dredd` category: 562 images (2000 AD, Mega City One, Lobo)
- `star_wars` category: 562 images (retro Star Wars technology, droids, terminals)
- Total: 5999 images across 8 categories (up from 6 categories in prior version)

**Verification after change:** `grep "judge_dredd\|star_wars" resources/themes/retro-bbc/wallpaper_downloader_final.py` must return both category definitions.

---

### 2. Fix the Script Deployment Bug in `software_tiers.rs`

#### Why This Needs to Change

**File:** `installer-core/src/software_tiers.rs`

The `install_wallpaper_downloader()` function (line 440 onward) has a **critical deployment gap**: it constructs a path for the Python script (`script_path`) and references it in the first-boot shell script, but **never writes the Python script content to that path**.

Current code:
```rust
fn install_wallpaper_downloader(ctx: &mut PhaseContext, home_dir: &std::path::Path) -> Result<()> {
    let script_path = home_dir.join(".local/bin/wallpaper_downloader_final.py");
    // ...
    // script_path is used in first_boot_script contents, but the .py file is NEVER written.
    // The first-boot shell script calls:  python3 "{downloader}" --first-boot
    // where {downloader} = script_path — but that file does not exist on disk.
```

Result: the first-boot systemd service fires, the shell script runs, calls `python3 ~/.local/bin/wallpaper_downloader_final.py --first-boot`, and immediately fails with `No such file or directory`.

#### 2.1 — Add `include_str!` Constant for the Script

**File:** `installer-core/src/software_tiers.rs`

Add at the top of the file, with the other `use` imports:

```rust
// Embedded wallpaper downloader script — deployed to ~/.local/bin at install time.
const WALLPAPER_DOWNLOADER_PY: &str =
    include_str!("../../resources/themes/retro-bbc/wallpaper_downloader_final.py");
```

This embeds the Python script into the binary at compile time, eliminating any runtime file-not-found dependency.

#### 2.2 — Write the Script to Disk in `install_wallpaper_downloader()`

**File:** `installer-core/src/software_tiers.rs`

**Location:** Inside `install_wallpaper_downloader()`, immediately after the `if ctx.options.dry_run { ... return Ok(()); }` block (currently around line 461).

**Add the following block after dry-run guard, before the pip install:**

```rust
// Deploy the Python downloader script.
if let Some(parent) = script_path.parent() {
    fs::create_dir_all(parent).context("creating wallpaper downloader bin directory")?;
}
fs::write(&script_path, WALLPAPER_DOWNLOADER_PY)
    .context("writing wallpaper_downloader_final.py")?;
{
    let mut perms = fs::metadata(&script_path)
        .context("reading wallpaper script permissions")?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)
        .context("setting wallpaper script permissions")?;
}
ctx.record_action(format!(
    "Deployed wallpaper_downloader_final.py to {}",
    script_path.display()
));
```

**What this fixes:** The Python file is now written to `~/.local/bin/wallpaper_downloader_final.py` with execute permissions before the first-boot service references it.

#### 2.3 — Update the Dry-Run Log to Reflect the Script Deploy

**File:** `installer-core/src/software_tiers.rs`

Inside the `if ctx.options.dry_run { ... }` block (around line 449), add a third log entry:

```rust
ctx.record_dry_run(
    "software_tiers",
    "Would deploy wallpaper_downloader_final.py script",
    Some(script_path.display().to_string()),
);
```

This makes dry-run output accurate — users can see the script deployment step in the dry-run summary.

#### 2.4 — Fix the `cmd::Command` Builder (Non-Standard API)

**File:** `installer-core/src/software_tiers.rs`

**Line ~462:**
```rust
cmd::Command::new("python3")
    .args(pip_cmd)
    .execute()
    .context("installing Python requests dependency")?;
```

Check whether `cmd::Command` with `.execute()` is the correct API in this codebase. Comparing to `installer-core/src/cmd.rs` usage elsewhere (e.g., `argon.rs`, `fonts.rs`) — all other modules use `std::process::Command` and `cmd::run(&mut cmd)`. If `cmd::Command::new(...).execute()` is not defined in `cmd.rs`, this will not compile.

**Fix if needed:** Replace with the standard pattern:
```rust
let mut pip = std::process::Command::new("python3");
pip.args(&pip_cmd);
cmd::run(&mut pip).context("installing Python requests dependency")?;
```

**How to verify:** `grep -n "fn execute\|pub fn execute" installer-core/src/cmd.rs` — if this returns nothing, the method does not exist and must be replaced.

---

### 3. Add Clean and Polish Phases (Improvement 2)

#### Why These Phases Don't Currently Fire

**File:** `installer-core/src/phase_registry.rs`

The current phase registry (as of Shaft J) contains:

| Phase Key | Gate | Status |
|---|---|---|
| `fonts` | Always | ✓ fires |
| `system_packages` | Always | ✓ fires |
| `software_tiers` | SoftwareTiers | ✓ fires if plan non-empty |
| `rust_toolchain` | Always | ✓ fires |
| `git_cli` | Always | ✓ fires |
| `buildroot_dependencies` | Profile(Dev) | ✓ fires Dev+ |
| `docker_engine` | Profile(Dev) | ✓ fires Dev+ |
| `shell_ux` | Profile(Dev) | ✓ fires Dev+ |
| `rclone` | Profile(Dev) | ✓ fires Dev+ |
| `snapshots` | Always | ✓ fires |
| `ai_spirits` | Always | ✓ fires |
| `pi4b_hdd_tuning` | Always | ✓ fires |
| `argon_one` | ModuleArgon | ✓ fires if enabled |
| `clean` | **MISSING** | ✗ no phase |
| `polish` | **MISSING** | ✗ no phase |

The clean phase (orphan removal, temp file cleanup) and polish phase (final config, wallpaper service activation) do not exist in the registry. They must be created as new modules and registered.

#### 3.1 — Create `installer-core/src/clean.rs`

**New file:** `installer-core/src/clean.rs`

This phase runs after all package installs. It removes orphaned packages and clears temp files.

```rust
use anyhow::Result;
use crate::{cmd, PhaseContext, PkgBackend};
use std::process::Command;

/// Remove orphaned packages and clear installer temp files.
pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    remove_orphans(ctx)?;
    clear_temp(ctx)?;
    Ok(())
}

fn remove_orphans(ctx: &mut PhaseContext) -> Result<()> {
    match ctx.platform.pkg_backend {
        PkgBackend::Pacman => {
            if ctx.options.dry_run {
                ctx.record_dry_run(
                    "clean",
                    "Would remove orphaned pacman packages",
                    Some("pacman -Rns $(pacman -Qdtq)".into()),
                );
                return Ok(());
            }
            // Get list of orphaned packages
            let orphans_out = Command::new("pacman")
                .args(["-Qdtq"])
                .output();
            match orphans_out {
                Ok(out) if !out.stdout.is_empty() => {
                    let orphan_list = String::from_utf8_lossy(&out.stdout);
                    let pkgs: Vec<&str> = orphan_list.split_whitespace().collect();
                    tracing::info!("Removing {} orphaned packages", pkgs.len());
                    let mut remove_cmd = Command::new("sudo");
                    remove_cmd.args(["pacman", "-Rns", "--noconfirm"]);
                    remove_cmd.args(&pkgs);
                    if let Err(e) = cmd::run(&mut remove_cmd) {
                        tracing::warn!("Orphan removal failed (non-fatal): {}", e);
                    } else {
                        ctx.record_action(format!("Removed {} orphaned packages", pkgs.len()));
                    }
                }
                Ok(_) => {
                    tracing::info!("No orphaned packages found.");
                }
                Err(e) => {
                    tracing::warn!("Could not query orphaned packages: {}", e);
                }
            }
        }
        PkgBackend::Apt => {
            if ctx.options.dry_run {
                ctx.record_dry_run(
                    "clean",
                    "Would autoremove unused apt packages",
                    Some("apt-get autoremove -y".into()),
                );
                return Ok(());
            }
            let mut cmd = Command::new("sudo");
            cmd.args(["apt-get", "autoremove", "-y"]);
            if let Err(e) = cmd::run(&mut cmd) {
                tracing::warn!("apt autoremove failed (non-fatal): {}", e);
            } else {
                ctx.record_action("Ran apt-get autoremove");
            }
        }
        PkgBackend::Dnf => {
            if ctx.options.dry_run {
                ctx.record_dry_run(
                    "clean",
                    "Would autoremove unused dnf packages",
                    Some("dnf autoremove -y".into()),
                );
                return Ok(());
            }
            let mut cmd = Command::new("sudo");
            cmd.args(["dnf", "autoremove", "-y"]);
            if let Err(e) = cmd::run(&mut cmd) {
                tracing::warn!("dnf autoremove failed (non-fatal): {}", e);
            } else {
                ctx.record_action("Ran dnf autoremove");
            }
        }
    }
    Ok(())
}

fn clear_temp(ctx: &mut PhaseContext) -> Result<()> {
    let staging_dir = &ctx.options.staging_dir;
    if ctx.options.dry_run {
        ctx.record_dry_run(
            "clean",
            "Would clear staging temp files",
            Some(staging_dir.display().to_string()),
        );
        return Ok(());
    }
    // Remove .tmp files from the staging directory only — never touch system paths
    if staging_dir.exists() {
        let mut removed = 0usize;
        if let Ok(entries) = std::fs::read_dir(staging_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) == Some("tmp") {
                    if let Err(e) = std::fs::remove_file(&p) {
                        tracing::warn!("Could not remove temp file {}: {}", p.display(), e);
                    } else {
                        removed += 1;
                    }
                }
            }
        }
        ctx.record_action(format!("Cleared {} temp files from staging dir", removed));
    }
    Ok(())
}
```

#### 3.2 — Create `installer-core/src/polish.rs`

**New file:** `installer-core/src/polish.rs`

This phase runs last. It activates the wallpaper systemd service (if present) and runs `fc-cache` to register newly installed fonts.

```rust
use anyhow::Result;
use crate::{cmd, PhaseContext};
use std::process::Command;

/// Final polish: font cache refresh, wallpaper service activation, marker write.
pub fn install_phase(ctx: &mut PhaseContext) -> Result<()> {
    refresh_font_cache(ctx)?;
    activate_wallpaper_service(ctx)?;
    Ok(())
}

fn refresh_font_cache(ctx: &mut PhaseContext) -> Result<()> {
    if ctx.options.dry_run {
        ctx.record_dry_run("polish", "Would refresh font cache", Some("fc-cache -f".into()));
        return Ok(());
    }
    let mut fc = Command::new("fc-cache");
    fc.arg("-f");
    if let Err(e) = cmd::run(&mut fc) {
        tracing::warn!("fc-cache failed (non-fatal): {}", e);
    } else {
        ctx.record_action("Font cache refreshed");
    }
    Ok(())
}

fn activate_wallpaper_service(ctx: &mut PhaseContext) -> Result<()> {
    let service = "mash-retro-wallpapers.service";
    if ctx.options.dry_run {
        ctx.record_dry_run(
            "polish",
            "Would enable mash-retro-wallpapers systemd user service",
            Some(format!("systemctl --user enable --now {}", service)),
        );
        return Ok(());
    }

    if !crate::systemd::is_available() {
        tracing::info!("systemd not available; skipping wallpaper service activation");
        return Ok(());
    }

    // Service unit is written by software_tiers::install_wallpaper_downloader().
    // Only try to enable it if the unit file actually exists.
    let service_path = dirs::home_dir()
        .map(|h| h.join(".config/systemd/user").join(service));

    let service_exists = service_path
        .as_ref()
        .map(|p| p.exists())
        .unwrap_or(false);

    if !service_exists {
        tracing::info!("Wallpaper service unit not found; skipping activation (retro theme not selected).");
        return Ok(());
    }

    let mut enable_cmd = Command::new("systemctl");
    enable_cmd.args(["--user", "enable", "--now", service]);
    if let Err(e) = cmd::run(&mut enable_cmd) {
        tracing::warn!("Could not enable wallpaper service (non-fatal): {}", e);
    } else {
        ctx.record_action(format!("Enabled systemd user service: {}", service));
    }
    Ok(())
}
```

#### 3.3 — Register Clean and Polish in `phase_registry.rs`

**File:** `installer-core/src/phase_registry.rs`

**Step 3.3.1 — Add module imports at the top:**

```rust
// ADD these two lines alongside the existing module imports:
use crate::clean;
use crate::polish;
```

**Step 3.3.2 — Add two new `PhaseGate` variants:**

```rust
// OLD:
enum PhaseGate {
    Always,
    Profile(ProfileLevel),
    ModuleArgon,
    SoftwareTiers,
}

// NEW:
enum PhaseGate {
    Always,
    Profile(ProfileLevel),
    ModuleArgon,
    SoftwareTiers,
    Clean,   // Always fires — orphan removal is safe on all profiles
    Polish,  // Always fires — fc-cache is a no-op if nothing new installed
}
```

**Step 3.3.3 — Add `should_run` arms for the new gates:**

```rust
// In impl PhaseGate fn should_run(), ADD:
PhaseGate::Clean  => true,
PhaseGate::Polish => true,
```

**Step 3.3.4 — Register the new phases at the END of the Default impl's vec:**

The phases must be appended **after** the existing `argon_one` entry. Order matters: clean before polish.

```rust
// ADD after the argon_one PhaseEntry:
PhaseEntry::new(
    "clean",
    "Clean",
    "Orphaned packages removed and temp files cleared",
    clean::install_phase,
    PhaseGate::Clean,
),
PhaseEntry::new(
    "polish",
    "Polish",
    "Font cache refreshed and post-install services activated",
    polish::install_phase,
    PhaseGate::Polish,
),
```

**Step 3.3.5 — Declare modules in `installer-core/src/lib.rs`:**

**File:** `installer-core/src/lib.rs`

Add alongside the other module declarations:

```rust
pub mod clean;
pub mod polish;
```

#### 3.4 — Update the Orchestrator Test Assertions

**File:** `installer-core/src/orchestrator.rs` (the `#[cfg(test)]` block)

The test `registry_minimal_profile_only_core_phases` checks that `Clean` and similar phases are NOT in Minimal but ARE in the expected list. Since both new phases are `PhaseGate::Always`, they fire on Minimal too.

**Update the expected list in both tests:**

```rust
// In registry_minimal_profile_only_core_phases:
let expected = vec![
    "System packages",
    "Filesystem Snapshots",
    "AI Spirits",
    "Rust toolchain + cargo tools",
    "Git, GitHub CLI, SSH",
    "Pi 4B HDD Tuning",
    "Clean",    // ← ADD
    "Polish",   // ← ADD
];
```

---

## 🏗️ File Touch Summary

| File | Section | Nature of Change |
|---|---|---|
| `resources/themes/retro-bbc/wallpaper_downloader_final.py` | §1.1 | Full replacement with 8-category final edition |
| `installer-core/src/software_tiers.rs` | §2.1, §2.2, §2.3, §2.4 | Add `include_str!` const; add script deployment block; add dry-run entry; fix `cmd` API |
| `installer-core/src/clean.rs` | §3.1 | **New file** — orphan removal + temp clear, per-backend |
| `installer-core/src/polish.rs` | §3.2 | **New file** — fc-cache + wallpaper service activation |
| `installer-core/src/phase_registry.rs` | §3.3 | Add `clean`/`polish` imports, two new `PhaseGate` variants, two `PhaseEntry` registrations |
| `installer-core/src/lib.rs` | §3.3.5 | Add `pub mod clean; pub mod polish;` |
| `installer-core/src/orchestrator.rs` | §3.4 | Update test expected phase lists |

---

## ⚠️ Risks & Mitigations

| Risk | Mitigation |
|---|---|
| `pacman -Qdtq` returns nothing → `pacman -Rns` called with empty args | Guard: check `!out.stdout.is_empty()` before constructing remove command |
| `clean` phase runs before all packages installed (wrong order) | Phase order in registry is sequential — `clean` is registered AFTER `argon_one` (last install phase) |
| `polish` fires wallpaper service when retro theme not selected | Guard: check `service_path.exists()` before calling `systemctl` |
| Wallpaper downloader requires `WALLHAVEN_API_KEY` — installer doesn't prompt for it | The script hardcodes `YOUR_API_KEY_HERE`; the first-boot service will fail silently (requests return 401). **Future Shaft** should add API key prompt to the TUI module selection screen |
| `include_str!` for Python script: file must exist at compile time | File is at a fixed path in the workspace; `cargo build` will fail loudly if missing |
| `cmd::Command` API mismatch in software_tiers.rs | Verified in §2.4 — replace with `std::process::Command` + `cmd::run()` pattern |
| New modules not added to `lib.rs` → compile error | §3.3.5 covers this explicitly |

---

## 📋 Wallhaven API Key — Future Work Note

The wallpaper downloader requires a valid Wallhaven API key in `WALLHAVEN_API_KEY`. The current script has `YOUR_API_KEY_HERE`. The first-boot systemd service will attempt to run and fail with HTTP 401 if the key is not replaced.

**This is not fixed in Shaft K.** The correct fix (a future shaft) is:
1. Add an "API Key" text input to the TUI module selection screen (new `ModuleState` field: `wallhaven_api_key: Option<String>`)
2. Pass the key through to `InstallOptions`
3. Write it into the deployed Python script via a `sed` replacement in `install_wallpaper_downloader()`

For now, the script is deployed correctly (bug fixed) and will work as soon as the user manually replaces the API key.

---

## ⚙️ Test Checklist

- [ ] `cargo build --workspace` passes after new files added
- [ ] `cargo test --workspace` passes — including updated orchestrator test assertions
- [ ] `cargo clippy --all-targets -- -D warnings` clean on `clean.rs` and `polish.rs`
- [ ] Dry-run run: output includes `"Would deploy wallpaper_downloader_final.py script"` entry
- [ ] Dry-run run: output includes `"Would remove orphaned pacman packages"` and `"Would refresh font cache"`
- [ ] Live run with retro theme: `~/.local/bin/wallpaper_downloader_final.py` exists and is executable
- [ ] Live run with retro theme: `systemctl --user status mash-retro-wallpapers.service` shows `enabled`
- [ ] Live run without retro theme: wallpaper service is NOT enabled
- [ ] `grep "judge_dredd\|star_wars" ~/.local/bin/wallpaper_downloader_final.py` returns both category blocks
- [ ] `fc-cache -f` is called during polish phase (visible in tracing log at INFO level)

---

**Status**: Planned ⏳
**Owner**: Bard
