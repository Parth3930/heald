use crate::okf::Document;
use std::path::Path;

pub fn run() {
    let local_base = std::env::current_dir().unwrap_or_default().join(".heald");
    let global_base = dirs::home_dir().unwrap_or_default().join(".heald");

    let mut has_errors = false;
    for base in &[local_base, global_base] {
        if base.exists() {
            println!("Checking bundle at {}...", base.display());
            has_errors |= check_dir(&base.join("rules"));
            has_errors |= check_dir(&base.join("skills"));
            has_errors |= check_dir(&base.join("memory"));
        }
    }

    if has_errors {
        println!("Doctor found issues.");
    } else {
        println!("Bundle is healthy.");
    }
}

fn check_dir(dir: &Path) -> bool {
    let mut has_errors = false;
    if !dir.exists() {
        return false;
    }
    
    // Check all markdown files recursively
    let walker = walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Err(e) = Document::from_file(path) {
                println!("ERROR in {}: {}", path.display(), e);
                has_errors = true;
            }
        }
    }
    has_errors
}
