use std::fs;
use std::path::Path;
use std::time::SystemTime;

const SCRATCH_DIR: &str = "docs/scratch";
const LEGACY_DIR: &str = "docs/legacy";
const INCOMING_DIR: &str = "docs/incoming-files";
const ASSETS_DIR: &str = "docs/assets";
const FORGE_TAVERN_DIR: &str = "docs/forge-tavern";

fn ensure_dirs(root: &Path) -> std::io::Result<()> {
    for dir in &[
        SCRATCH_DIR,
        LEGACY_DIR,
        INCOMING_DIR,
        ASSETS_DIR,
        FORGE_TAVERN_DIR,
    ] {
        fs::create_dir_all(root.join(dir))?;
    }
    println!("Directories verified.");
    Ok(())
}

fn file_age_days(path: &Path) -> u64 {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|modified| SystemTime::now().duration_since(modified).ok())
        .map(|d| d.as_secs() / 86400)
        .unwrap_or(0)
}

fn move_old_scratch_files(root: &Path) -> std::io::Result<()> {
    println!("Moving files older than 7 days from scratch to legacy...");
    let scratch = root.join(SCRATCH_DIR);
    let legacy = root.join(LEGACY_DIR);
    let mut moved = 0usize;

    if let Ok(entries) = fs::read_dir(&scratch) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && file_age_days(&path) > 7 {
                let dest = legacy.join(path.file_name().unwrap());
                println!("  Moving: {} -> {}", path.display(), dest.display());
                if fs::rename(&path, &dest).is_err() {
                    fs::copy(&path, &dest)?;
                    fs::remove_file(&path)?;
                }
                moved += 1;
            }
        }
    }

    if moved == 0 {
        println!("  Nothing to move (no scratch files older than 7 days).");
    } else {
        println!("  Moved {} file(s) to legacy.", moved);
    }
    Ok(())
}

fn count_files(dir: &Path) -> usize {
    fs::read_dir(dir)
        .map(|entries| entries.flatten().filter(|e| e.path().is_file()).count())
        .unwrap_or(0)
}

fn remove_empty_subdirs(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let is_empty = fs::read_dir(&path)
                    .map(|mut e| e.next().is_none())
                    .unwrap_or(false);
                if is_empty {
                    let _ = fs::remove_dir(&path);
                    println!("  Removed empty dir: {}", path.display());
                }
            }
        }
    }
}

fn show_hierarchy() {
    println!("\ndocs/");
    println!("├── assets/              # All asset files (images, binaries)");
    println!("├── forge-tavern/        # Four sources of truth (IMMUTABLE)");
    println!("│   ├── bard-bbs-profile.md  # Comprehensive bio");
    println!("│   ├── bard-quick-ref.md    # Cheatsheet");
    println!("│   ├── maps.md              # Current work");
    println!("│   └── maps-explored.md     # Completed sessions");
    println!("├── incoming-files/      # Staging for new docs");
    println!("├── legacy/              # Archived (>7 days from scratch)");
    println!("├── scratch/             # Temporary work notes (<7 days)");
    println!("├── HISTORY.md           # Tales and journal");
    println!("├── MANUAL.md            # User guide");
    println!("└── LICENSE              # Legal");
}

pub fn run(_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Document Hygiene...\n");
    let root = crate::project_root();

    ensure_dirs(&root)?;
    move_old_scratch_files(&root)?;

    println!("\nDocument counts:");
    println!("  scratch:      {}", count_files(&root.join(SCRATCH_DIR)));
    println!("  legacy:       {}", count_files(&root.join(LEGACY_DIR)));
    println!("  incoming:     {}", count_files(&root.join(INCOMING_DIR)));
    println!("  assets:       {}", count_files(&root.join(ASSETS_DIR)));
    println!(
        "  forge-tavern: {}",
        count_files(&root.join(FORGE_TAVERN_DIR))
    );

    println!("\nCleaning empty scratch subdirs...");
    remove_empty_subdirs(&root.join(SCRATCH_DIR));

    show_hierarchy();
    println!("\nDocument Hygiene Complete.");
    Ok(())
}
