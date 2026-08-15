use chrono::Utc;
use std::io::Read;

pub fn run(doc_type: &str, title: &str, body_opt: Option<&str>, stdin: bool) {
    let local_base = std::env::current_dir().unwrap_or_default().join(".heald");
    
    // Auto-init if local workspace doesn't exist
    if !local_base.exists() {
        crate::cmd::init::run(false);
    }
    
    let memory_dir = local_base.join("memory");
    if !memory_dir.exists() {
        std::fs::create_dir_all(&memory_dir).unwrap();
    }

    let timestamp = Utc::now().to_rfc3339();
    let slug = title.to_lowercase().replace(" ", "-").chars().filter(|c| c.is_alphanumeric() || *c == '-').collect::<String>();
    
    let mut body = String::new();
    if let Some(b) = body_opt {
        body.push_str(b);
    } else if stdin {
        std::io::stdin().read_to_string(&mut body).unwrap();
    }
    
    if body.trim().is_empty() {
        eprintln!("ERROR (Heald): You must provide a body! Example: heald remember --type decision --title '...' --body \"What you decided...\"");
        std::process::exit(1);
    }

    let content = format!(
        "---\ntype: {}\ntitle: \"{}\"\ntimestamp: {}\n---\n{}\n",
        doc_type, title, timestamp, body
    );

    let path = memory_dir.join(format!("{}.md", slug));
    std::fs::write(&path, content).unwrap();
    crate::xref::rebuild_memory_index(&local_base);
    println!("Saved memory to {}", path.display());
}
