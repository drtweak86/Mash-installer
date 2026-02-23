use std::fs;
use std::path::Path;

struct Results {
    pass: usize,
    fail: usize,
}

impl Results {
    fn new() -> Self {
        Self { pass: 0, fail: 0 }
    }

    fn check_file(&mut self, root: &Path, rel: &str) -> bool {
        let path = root.join(rel);
        let exists = path.exists();
        if exists {
            println!("  PASS  {}", rel);
            self.pass += 1;
        } else {
            println!("  MISS  {}", rel);
            self.fail += 1;
        }
        exists
    }

    fn check_contains(&mut self, root: &Path, file: &str, needle: &str) -> bool {
        let path = root.join(file);
        match fs::read_to_string(&path) {
            Ok(content) => {
                let found = content.contains(needle);
                if found {
                    println!("    PASS  '{}' in {}", needle, file);
                    self.pass += 1;
                } else {
                    println!("    MISS  '{}' in {}", needle, file);
                    self.fail += 1;
                }
                found
            }
            Err(_) => {
                println!("    MISS  '{}' (file not readable: {})", needle, file);
                self.fail += 1;
                false
            }
        }
    }
}

pub fn run(_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Bard's Theme Testing Tavern");
    println!("===========================\n");

    let root = crate::project_root();
    let mut r = Results::new();

    // Test 1: Theme resource files
    println!("Test 1: Theme File Verification");
    println!("--------------------------------");
    for file in &[
        "resources/themes/retro-bbc/i3-config",
        "resources/themes/retro-bbc/i3status-retro.conf",
        "resources/themes/retro-bbc/kitty.conf",
        "resources/themes/retro-bbc/conkyrc",
    ] {
        r.check_file(&root, file);
    }

    // Test 2: Theme module structure
    println!("\nTest 2: Theme Module Structure");
    println!("--------------------------------");
    r.check_file(&root, "installer-core/src/theme.rs");
    r.check_contains(&root, "installer-core/src/theme.rs", "install_retro_theme");
    r.check_contains(
        &root,
        "installer-core/src/theme.rs",
        "ensure_retro_theme_dependencies",
    );

    // Test 3: Menu integration
    println!("\nTest 3: Menu Integration");
    println!("------------------------");
    r.check_file(&root, "installer-cli/src/menu.rs");
    r.check_contains(&root, "installer-cli/src/menu.rs", "run_theme_menu");
    r.check_contains(&root, "installer-cli/src/menu.rs", "ThemePlan::RetroOnly");

    // Test 4: Software tiers
    println!("\nTest 4: Software Tiers Integration");
    println!("------------------------------------");
    r.check_file(&root, "installer-core/src/software_tiers.rs");
    r.check_contains(
        &root,
        "installer-core/src/software_tiers.rs",
        "pub enum ThemePlan",
    );
    r.check_contains(
        &root,
        "installer-core/src/software_tiers.rs",
        "theme_plan: ThemePlan",
    );

    // Test 5: Library exports
    println!("\nTest 5: Library Exports");
    println!("-----------------------");
    r.check_file(&root, "installer-core/src/lib.rs");
    r.check_contains(&root, "installer-core/src/lib.rs", "pub use theme::");

    // Test 6: Integration test file
    println!("\nTest 6: Integration Tests");
    println!("--------------------------");
    r.check_file(&root, "installer-core/tests/theme_integration.rs");

    // Summary
    println!("\nTest Summary");
    println!("============");
    println!("  PASS: {}  MISS: {}", r.pass, r.fail);
    if r.fail == 0 {
        println!("All checks passed. Ready for Rust compilation and testing.");
        Ok(())
    } else {
        println!(
            "{} check(s) failed. See MISS entries above — these features may not be implemented yet.",
            r.fail
        );
        std::process::exit(1);
    }
}
