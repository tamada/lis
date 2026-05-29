use lis::LisBuilder;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_list_basic_structure() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");
    let sub = dir.path().join("sub");
    
    fs::write(&file1, "hello").unwrap();
    fs::write(&file2, "world").unwrap();
    fs::create_dir(&sub).unwrap();

    let lis = LisBuilder::new().build(dir.path());
    let entries = lis.list();
    
    assert_eq!(entries.len(), 3);
    assert!(entries.iter().any(|e| e.name == "file1.txt"));
    assert!(entries.iter().any(|e| e.name == "file2.txt"));
    assert!(entries.iter().any(|e| e.name == "sub"));
}

#[test]
fn test_list_all_hidden() {
    let dir = tempdir().unwrap();
    let hidden = dir.path().join(".hidden");
    let normal = dir.path().join("normal.txt");
    
    fs::write(&hidden, "hidden").unwrap();
    fs::write(&normal, "normal").unwrap();

    // Default: no hidden
    let lis = LisBuilder::new().build(dir.path());
    let entries = lis.list();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "normal.txt");

    // All: show hidden
    let lis_all = LisBuilder::new().all(true).build(dir.path());
    let entries_all = lis_all.list();
    assert_eq!(entries_all.len(), 2);
    assert!(entries_all.iter().any(|e| e.name == ".hidden"));
}

#[test]
fn test_recursive_listing() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    let file = sub.join("file.txt");
    
    fs::create_dir(&sub).unwrap();
    fs::write(&file, "hello").unwrap();

    // Not recursive
    let lis = LisBuilder::new().build(dir.path());
    let entries = lis.list();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "sub");

    // Recursive
    let lis_rec = LisBuilder::new().recursive(true).build(dir.path());
    let entries_rec = lis_rec.list();
    assert_eq!(entries_rec.len(), 3);
    assert!(entries_rec.iter().any(|e| e.name == "sub"));
    assert!(entries_rec.iter().any(|e| e.name == "sub/file.txt" || e.name == "sub\\file.txt"));
}

#[test]
#[cfg(unix)]
fn test_symlink_handling() {
    use std::os::unix::fs::symlink;
    
    let dir = tempdir().unwrap();
    let target = dir.path().join("target.txt");
    let link = dir.path().join("link.txt");
    
    fs::write(&target, "target").unwrap();
    symlink(&target, &link).unwrap();

    let lis = LisBuilder::new().build(dir.path());
    let entries = lis.list();
    
    let link_entry = entries.iter().find(|e| e.name == "link.txt").unwrap();
    assert!(link_entry.mode.starts_with('l'));
}

#[test]
fn test_sorting_by_extension() {
    use lis::entry::{sort_entries, SortBy};
    
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("b.txt"), "").unwrap();
    fs::write(dir.path().join("a.rs"), "").unwrap();
    fs::write(dir.path().join("c.md"), "").unwrap();

    let lis = LisBuilder::new().build(dir.path());
    let mut entries = lis.list();
    
    sort_entries(&mut entries, SortBy::Extension, false);
    
    assert_eq!(entries[0].extension, "md");
    assert_eq!(entries[1].extension, "rs");
    assert_eq!(entries[2].extension, "txt");
}

#[test]
fn test_gitignore_respect() {
    use std::process::Command;

    let dir = tempdir().unwrap();
    
    // Initialize a git repo so that the ignore crate picks up .gitignore reliably
    Command::new("git").arg("init").arg(dir.path()).status().unwrap();
    
    let gitignore = dir.path().join(".gitignore");
    let ignored_file = dir.path().join("ignored.txt");
    let normal_file = dir.path().join("normal.txt");
    
    fs::write(&gitignore, "ignored.txt").unwrap();
    fs::write(&ignored_file, "content").unwrap();
    fs::write(&normal_file, "content").unwrap();

    let lis = LisBuilder::new().recursive(true).build(dir.path());
    let entries = lis.list();
    assert!(!entries.iter().any(|e| e.name == "ignored.txt"));
    assert!(entries.iter().any(|e| e.name == "normal.txt"));

    // whole_all: ignore gitignore
    let lis_whole = LisBuilder::new().recursive(true).whole_all(true).build(dir.path());
    let entries_whole = lis_whole.list();
    assert!(entries_whole.iter().any(|e| e.name == "ignored.txt"));
}

#[test]
fn test_git_status_integration() {
    use std::process::Command;

    let dir = tempdir().unwrap();
    
    // Initialize a git repo
    let status = Command::new("git")
        .arg("init")
        .arg(dir.path())
        .status()
        .unwrap();
    if !status.success() {
        return; // Skip if git is not available or failed to init
    }

    let file = dir.path().join("new_file.txt");
    fs::write(&file, "content").unwrap();

    let lis = LisBuilder::new().build(dir.path());
    let entries = lis.list();
    
    let entry = entries.iter().find(|e| e.name == "new_file.txt").unwrap();
    // It should be 'A' for a new untracked file (with include_untracked=true)
    assert_eq!(entry.git_status, "A");
}
