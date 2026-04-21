use lis::Lis;
use lis::entry::{SortBy, sort_entries};
use std::path::PathBuf;

fn main() {
    // Advanced usage: recursive, including all files, sorted by size descending
    let mut entries = Lis::new(PathBuf::from("."))
        .recursive(true)
        .all(true)
        .list();

    // Sort by size, reverse
    sort_entries(&mut entries, SortBy::Size, true);

    println!("{:<10} {:>10} {:<20} Path", "Mode", "Size", "Modified");
    println!("{:-<60}", "");

    for entry in entries.iter().take(10) {
        println!(
            "{:<10} {:>10} {:<20} {}",
            entry.mode, entry.size, entry.modified, entry.name
        );
    }
}
