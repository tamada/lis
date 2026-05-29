use lis::Lis;
use std::path::PathBuf;

#[test]
fn test_list_current_dir() {
    let lis = Lis::new(PathBuf::from("."));
    let entries = lis.list();
    
    // Check that we have some entries (at least Cargo.toml, src, etc.)
    assert!(!entries.is_empty());
    
    // Check if Cargo.toml is in the list
    let has_cargo_toml = entries.iter().any(|e| e.name == "Cargo.toml");
    assert!(has_cargo_toml);
}

#[test]
fn test_list_src_dir() {
    let lis = Lis::new(PathBuf::from("src"));
    let entries = lis.list();
    
    assert!(!entries.is_empty());
    
    // Check if lib.rs is in the list
    let has_lib_rs = entries.iter().any(|e| e.name == "lib.rs");
    assert!(has_lib_rs);
}

#[test]
fn test_recursive_list() {
    let lis = Lis::new(PathBuf::from(".")).recursive(true);
    let entries = lis.list();
    
    assert!(!entries.is_empty());
    
    // In a recursive list, we should see paths like "src/lib.rs" or "cli/main.rs"
    // The implementation of get_display_name for recursive:
    // Some(if s.is_empty() { self.path.to_string_lossy().into_owned() } else { s })
    
    let has_src_lib_rs = entries.iter().any(|e| e.name == "src/lib.rs" || e.name == "src/lib.rs".replace("/", std::path::MAIN_SEPARATOR.to_string().as_str()));
    assert!(has_src_lib_rs);
}

#[test]
fn test_list_all() {
    let lis = Lis::new(PathBuf::from(".")).all(true);
    let entries = lis.list();
    
    // Check if .gitignore is in the list (it should be as it's a hidden file but not ignored by itself usually)
    let has_gitignore = entries.iter().any(|e| e.name == ".gitignore");
    assert!(has_gitignore);
}

#[test]
fn test_sorting_integration() {
    use lis::entry::{sort_entries, SortBy};
    
    let lis = Lis::new(PathBuf::from("."));
    let mut entries = lis.list();
    
    sort_entries(&mut entries, SortBy::Size, false);
    for i in 0..entries.len() - 1 {
        assert!(entries[i].size <= entries[i+1].size);
    }
}
