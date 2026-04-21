use lis::Lis;
use std::path::PathBuf;

fn main() {
    // Basic usage: list the current directory
    let entries = Lis::new(PathBuf::from(".")).list();

    println!("Listing current directory:");
    for entry in entries {
        println!("{:<10} {}", entry.mode, entry.name);
    }
}
