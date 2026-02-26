use std::fs;
use std::path::{Path, PathBuf};

fn is_external(link: &str) -> bool {
    link.starts_with("http://")
        || link.starts_with("https://")
        || link.starts_with("mailto:")
        || link.starts_with("ftp://")
        || link.starts_with("javascript:")
}

fn extract_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let bytes = content.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b']' && i + 1 < bytes.len() && bytes[i + 1] == b'(' {
            i += 2;
            let start = i;
            while i < bytes.len() && bytes[i] != b')' && bytes[i] != b'\n' {
                i += 1;
            }
            if i < bytes.len() && bytes[i] == b')' {
                links.push(content[start..i].trim().to_string());
            }
        }
        i += 1;
    }
    links
}

fn collect_md_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip scratch/ and legacy/ — temporary/archived files are not subject to link checks
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "scratch" || name == "legacy" {
                    continue;
                }
                collect_md_files(&path, files);
            } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                files.push(path);
            }
        }
    }
}

/// Core check logic — returns true if all links are valid.
/// Called by release_check::run() as well as directly via `cargo xtask check-docs`.
pub fn check(root: &Path) -> bool {
    let docs_dir = root.join("docs");

    if !docs_dir.exists() {
        eprintln!("docs/ directory not found at {}", docs_dir.display());
        return false;
    }

    let mut md_files = Vec::new();
    collect_md_files(&docs_dir, &mut md_files);

    let mut missing: Vec<(PathBuf, String)> = Vec::new();

    for md_path in &md_files {
        let content = match fs::read_to_string(md_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for raw_link in extract_links(&content) {
            let link = raw_link.trim();
            if link.is_empty() || link.starts_with('#') || is_external(link) {
                continue;
            }
            let link_path = link.split('#').next().unwrap_or("").trim();
            if link_path.is_empty() {
                continue;
            }
            let target = md_path.parent().unwrap().join(link_path);
            if !target.exists() {
                let rel = md_path.strip_prefix(root).unwrap_or(md_path);
                missing.push((rel.to_path_buf(), link.to_string()));
            }
        }
    }

    if missing.is_empty() {
        println!("Documentation link check passed.");
        true
    } else {
        println!("Broken documentation references detected:");
        for (source, target) in &missing {
            println!("  {} -> {}", source.display(), target);
        }
        false
    }
}

pub fn run(_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let root = crate::project_root();
    if check(&root) {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
