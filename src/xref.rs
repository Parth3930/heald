use std::path::{Path, PathBuf};
use crate::okf::Document;
use chrono::{DateTime, Utc};

pub struct Reference {
    pub title: String,
    pub timestamp: String,
    pub snippet: String,
    pub doc_filename: String,
}

pub fn get_memory_docs(local_base: &Path) -> Vec<(PathBuf, Document)> {
    let memory_dir = local_base.join("memory");
    let mut docs = Vec::new();
    if memory_dir.exists() {
        let walker = walkdir::WalkDir::new(&memory_dir).into_iter().filter_map(|e| e.ok());
        for entry in walker {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(doc) = Document::from_file(path) {
                    if path.file_name().and_then(|n| n.to_str()) != Some("index.md") &&
                       path.file_name().and_then(|n| n.to_str()) != Some("log.md") {
                        docs.push((path.to_path_buf(), doc));
                    }
                }
            }
        }
    }
    // Sort by timestamp descending
    docs.sort_by(|a, b| {
        let ts_a = a.1.frontmatter.timestamp.as_deref().unwrap_or("");
        let ts_b = b.1.frontmatter.timestamp.as_deref().unwrap_or("");
        let a_time = DateTime::parse_from_rfc3339(ts_a).map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now());
        let b_time = DateTime::parse_from_rfc3339(ts_b).map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now());
        b_time.cmp(&a_time)
    });
    docs
}

pub fn normalize_path(path: &str) -> String {
    let path = path.replace("\\", "/");
    let path = path.strip_prefix("./").unwrap_or(&path).to_string();
    path
}

pub fn extract_snippet(content: &str, path: &str) -> Option<String> {
    let path_norm = normalize_path(path);
    for line in content.lines() {
        if line.contains(&path_norm) {
            let mut snippet = line.trim().to_string();
            if snippet.len() > 100 {
                snippet.truncate(97);
                snippet.push_str("...");
            }
            return Some(snippet);
        }
    }
    None
}

pub fn find_references(docs: &[(PathBuf, Document)], file_path: &str) -> Vec<Reference> {
    let mut refs = Vec::new();
    let norm = normalize_path(file_path);
    for (doc_path, doc) in docs {
        if let Some(snippet) = extract_snippet(&doc.content, &norm) {
            let doc_filename = doc_path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let title = doc.frontmatter.title.clone().unwrap_or_else(|| doc_filename.clone());
            let timestamp = doc.frontmatter.timestamp.clone().unwrap_or_else(|| "".to_string());
            // simplify timestamp to just date for display
            let ts = if timestamp.len() >= 10 { timestamp[..10].to_string() } else { timestamp };
            refs.push(Reference {
                title,
                timestamp: ts,
                snippet,
                doc_filename,
            });
        }
    }
    refs
}
