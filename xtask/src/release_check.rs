use std::process::Command;

fn banner(title: &str) {
    println!("\n==== RELEASE CHECKLIST: {} ====", title);
}

fn run_cmd(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn run(_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let root = crate::project_root();

    banner("Ensure repository is clean");
    if !run_cmd("git", &["status", "-sb"]) {
        eprintln!("git status failed");
        std::process::exit(1);
    }

    banner("Verify documentation integrity");
    if !crate::check_docs::check(&root) {
        eprintln!("Documentation check failed — fix broken links before release.");
        std::process::exit(1);
    }

    banner("cargo fmt --check");
    if !run_cmd("cargo", &["fmt", "--all", "--", "--check"]) {
        eprintln!("cargo fmt check failed — run `cargo fmt --all` first.");
        std::process::exit(1);
    }

    banner("cargo clippy");
    if !run_cmd(
        "cargo",
        &[
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    ) {
        eprintln!("clippy found warnings — fix them before release.");
        std::process::exit(1);
    }

    banner("cargo test --workspace");
    if !run_cmd("cargo", &["test", "--workspace"]) {
        eprintln!("Tests failed — all tests must pass before release.");
        std::process::exit(1);
    }

    banner("Manual release reminders");
    println!("  - Update docs (MANUAL.md) and ensure all links are fresh.");
    println!("  - Update HISTORY.md with a bardic release entry.");
    println!("  - Bump version: cargo xtask bump <patch|minor|major>");
    println!("  - Run dry-run: `mash-setup --dry-run` or `mash-setup doctor`.");
    println!("  - Tag the release: git tag v<VERSION> && git push --tags");
    println!("  - Deploy documentation.");

    println!("\nRelease checklist complete. The forge is ready to ship.");
    Ok(())
}
