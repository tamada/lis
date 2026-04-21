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
