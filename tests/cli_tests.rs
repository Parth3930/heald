use std::process::Command;
use std::fs;
use std::path::PathBuf;

fn get_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_heald"))
}

#[test]
fn test_blame_one_file() {
    // Before testing, let's create a fake memory document
    let local_base = PathBuf::from(".heald");
    let mem_dir = local_base.join("memory");
    fs::create_dir_all(&mem_dir).unwrap();
    fs::write(mem_dir.join("test-doc-1.md"), "---\ntitle: Test Doc 1\ntimestamp: 2026-08-09T00:00:00Z\ntype: decision\n---\nWe decided to use src/auth.rs for auth.\n").unwrap();
    
    let output = Command::new(get_bin())
        .args(&["blame", "src/auth.rs"])
        .output()
        .expect("failed to execute");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("src/auth.rs"));
    assert!(stdout.contains("test-doc-1.md"));
    assert!(stdout.contains("We decided to use src/auth.rs for auth."));
}

#[test]
fn test_blame_multiple_docs_ordering() {
    let local_base = PathBuf::from(".heald");
    let mem_dir = local_base.join("memory");
    fs::create_dir_all(&mem_dir).unwrap();
    // Older
    fs::write(mem_dir.join("test-doc-old.md"), "---\ntitle: Old Doc\ntimestamp: 2025-01-01T00:00:00Z\ntype: decision\n---\nRef src/auth.rs older.\n").unwrap();
    // Newer
    fs::write(mem_dir.join("test-doc-new.md"), "---\ntitle: New Doc\ntimestamp: 2026-08-10T00:00:00Z\ntype: decision\n---\nRef src/auth.rs newer.\n").unwrap();

    let output = Command::new(get_bin())
        .args(&["blame", "src/auth.rs"])
        .output()
        .expect("failed to execute");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    
    // Check ordering: new should appear before old
    let idx_new = stdout.find("test-doc-new.md").unwrap();
    let idx_old = stdout.find("test-doc-old.md").unwrap();
    assert!(idx_new < idx_old);
}

#[test]
fn test_blame_zero_refs() {
    let output = Command::new(get_bin())
        .args(&["blame", "src/nonexistent.rs"])
        .output()
        .expect("failed to execute");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("No decisions on record for src/nonexistent.rs"));
}

#[test]
fn test_blame_directory() {
    // Create another file under src/cmd
    let local_base = PathBuf::from(".heald");
    let mem_dir = local_base.join("memory");
    fs::create_dir_all(&mem_dir).unwrap();
    fs::write(mem_dir.join("test-doc-cmd.md"), "---\ntimestamp: 2026-08-10T00:00:00Z\ntype: decision\n---\nWe touch src/cmd/map.rs here.\n").unwrap();

    let output = Command::new(get_bin())
        .args(&["blame", "src/cmd"])
        .output()
        .expect("failed to execute");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("src/cmd/map.rs"));
    assert!(stdout.contains("test-doc-cmd.md"));
}

#[test]
fn test_blame_json() {
    let output = Command::new(get_bin())
        .args(&["blame", "src/auth.rs", "--json"])
        .output()
        .expect("failed to execute");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    // Should be parseable JSON
    let val: serde_json::Value = serde_json::from_str(&stdout).expect("Valid JSON");
    assert!(val.is_object());
    let obj = val.as_object().unwrap();
    assert!(obj.contains_key("src/auth.rs"));
    assert!(obj["src/auth.rs"].as_array().unwrap().len() >= 2);
}

#[test]
fn test_map_against_repo() {
    // Run map
    let output = Command::new(get_bin())
        .args(&["map"])
        .output()
        .expect("failed to execute");
    
    assert!(output.status.success());

    // Read the map file
    let map_file = PathBuf::from(".heald/map.md");
    assert!(map_file.exists());
    let content = fs::read_to_string(map_file).unwrap();

    // Check token cap: 8000 chars should be safely under ~2000 tokens. 
    // Wait, the code caps around 6000 chars and then truncates.
    assert!(content.len() <= 6500, "Map is too large: {} bytes", content.len());

    // Must include at least one cross-referenced memory doc.
    assert!(content.contains("see "), "Must contain cross reference");
}

#[test]
fn test_context_and_memory_index() {
    let local_base = PathBuf::from(".heald");
    let mem_dir = local_base.join("memory");
    fs::create_dir_all(&mem_dir).unwrap();
    fs::write(
        mem_dir.join("test-auto-index.md"),
        "---\ntitle: Auto Index Test\ntimestamp: 2026-08-15T00:00:00Z\ntype: decision\ntags: [pinned]\n---\nAuto generated index test content.\n",
    ).unwrap();

    let output = Command::new(get_bin())
        .args(&["context", "agents"])
        .output()
        .expect("failed to execute");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Project Memory Manifest"));
    assert!(stdout.contains("Auto Index Test"));
    assert!(stdout.contains("test-auto-index.md"));

    let index_file = mem_dir.join("index.md");
    assert!(index_file.exists());
    let index_content = fs::read_to_string(index_file).unwrap();
    assert!(index_content.contains("Project Memory Manifest"));
    assert!(index_content.contains("Auto Index Test"));
}

