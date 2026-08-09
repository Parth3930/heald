use crate::okf::Document;

pub fn run(_harness: &str, budget: Option<usize>) {
    let local_base = std::env::current_dir().unwrap_or_default().join(".heald");
    
    // Auto-init if local workspace doesn't exist
    if !local_base.exists() {
        crate::cmd::init::run(false);
    }

    let memory_dir = local_base.join("memory");

    let mut docs = Vec::new();
    let mut index_doc = None;

    if memory_dir.exists() {
        let walker = walkdir::WalkDir::new(&memory_dir)
            .into_iter()
            .filter_map(|e| e.ok());
        for entry in walker {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(doc) = Document::from_file(path) {
                    if path.file_name().and_then(|n| n.to_str()) == Some("index.md") {
                        index_doc = Some(doc);
                    } else if path.file_name().and_then(|n| n.to_str()) != Some("log.md") {
                        docs.push(doc);
                    }
                }
            }
        }
    }

    docs.sort_by_key(|d| -score(d));

    let budget_chars = budget.unwrap_or(8000) * 4;
    let mut current_chars = 0;
    let mut included = Vec::new();

    if let Some(idx) = index_doc {
        let len = idx.content.len();
        current_chars += len;
        included.push(idx);
    }

    let total_available = docs.len() + if included.is_empty() { 0 } else { 1 };

    for doc in docs {
        let len = doc.content.len();
        if current_chars + len <= budget_chars {
            current_chars += len;
            included.push(doc);
        }
    }

    println!(
        "<!-- Heald Context: Included {}/{} memory documents (Budget: {} chars) -->",
        included.len(),
        total_available,
        budget_chars
    );
    for doc in included {
        if let Some(title) = &doc.frontmatter.title {
            println!("\n# {}\n\n{}", title, doc.content);
        } else {
            println!("\n# {}\n\n{}", doc.frontmatter.r#type, doc.content);
        }
    }
}

fn score(doc: &Document) -> i32 {
    let mut s = 0;
    if let Some(tags) = &doc.frontmatter.tags {
        if tags.iter().any(|t| t == "pinned") {
            s += 1000;
        }
    }
    if let Some(ts) = &doc.frontmatter.timestamp {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts) {
            let days = (chrono::Utc::now() - dt.with_timezone(&chrono::Utc)).num_days();
            s += (100 - days).max(0) as i32;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_pinned() {
        let doc_pinned = Document {
            frontmatter: crate::okf::Frontmatter {
                r#type: "decision".to_string(),
                title: None,
                description: None,
                resource: None,
                tags: Some(vec!["pinned".to_string()]),
                timestamp: None,
            },
            content: "test".to_string(),
        };
        let doc_unpinned = Document {
            frontmatter: crate::okf::Frontmatter {
                r#type: "decision".to_string(),
                title: None,
                description: None,
                resource: None,
                tags: None,
                timestamp: None,
            },
            content: "test".to_string(),
        };
        assert!(score(&doc_pinned) > score(&doc_unpinned) + 500);
    }
}
