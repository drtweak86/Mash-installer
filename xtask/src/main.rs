mod branch_prune;
mod bump;
mod check_docs;
mod hygiene;
mod release_check;
mod test_infra;
mod test_theme;

pub fn project_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn usage() {
    println!("Usage: cargo xtask <subcommand>");
    println!();
    println!("Subcommands:");
    println!("  check-docs    Check for broken markdown links in docs/");
    println!("  bump          Bump version: cargo xtask bump <patch|minor|major>");
    println!("  release-check Run pre-release gate (fmt + clippy + tests + docs)");
    println!("  hygiene       Move old scratch docs to legacy/");
    println!("  branch-prune  Prune local branches older than 7 days");
    println!("  test-infra    Run test infrastructure (maelstrom|hardware modes)");
    println!("  test-theme    Run theme integration checks");
}

fn main() {
    let mut args = std::env::args().skip(1);
    let subcommand = match args.next() {
        Some(s) => s,
        None => {
            usage();
            std::process::exit(1);
        }
    };
    let rest: Vec<String> = args.collect();

    let result: Result<(), Box<dyn std::error::Error>> = match subcommand.as_str() {
        "check-docs" => check_docs::run(&rest),
        "bump" => bump::run(&rest),
        "release-check" => release_check::run(&rest),
        "hygiene" => hygiene::run(&rest),
        "branch-prune" => branch_prune::run(&rest),
        "test-infra" => test_infra::run(&rest),
        "test-theme" => test_theme::run(&rest),
        _ => {
            eprintln!("Unknown subcommand: {}", subcommand);
            usage();
            std::process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
