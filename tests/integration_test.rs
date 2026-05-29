use lis::Lis;
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

    let lis = Lis::new(dir.path().to_path_buf());
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
    let lis = Lis::new(dir.path().to_path_buf());
    let entries = lis.list();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "normal.txt");

    // All: show hidden
    let lis_all = Lis::new(dir.path().to_path_buf()).all(true);
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
    let lis = Lis::new(dir.path().to_path_buf());
    let entries = lis.list();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "sub");

    // Recursive
    let lis_rec = Lis::new(dir.path().to_path_buf()).recursive(true);
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

    let lis = Lis::new(dir.path().to_path_buf());
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

    let lis = Lis::new(dir.path().to_path_buf());
    let mut entries = lis.list();
    
    sort_entries(&mut entries, SortBy::Extension, false);
    
    assert_eq!(entries[0].extension, "md");
    assert_eq!(entries[1].extension, "rs");
    assert_eq!(entries[2].extension, "txt");
}
