---
title: "Library Usage"
date: 2026-06-09
draft: false
---

# 📖 Library Usage

`lis` can be integrated as a library into your own Rust projects.

## 🛠️ Basic Usage

To list entries in the current directory:

```rust
use lis::Lis;
use std::path::PathBuf;

fn main() {
    let entries = Lis::new(PathBuf::from(".")).list();

    for entry in entries {
        println!("{:<10} {}", entry.mode, entry.name);
    }
}
```

## 🏗️ Using LisBuilder

For more control over the listing behavior, use `LisBuilder`.

```rust
use lis::LisBuilder;

fn main() {
    let lis = LisBuilder::new()
        .all(true)       // Include hidden files
        .recursive(false)
        .build(".");

    let entries = lis.list();
    for entry in entries {
        println!("{}: {}", entry.name, entry.git_status);
    }
}
```

## 📊 Advanced Sorting

You can also use the sorting utilities provided by the library.

```rust
use lis::entry::{SortBy, sort_entries};

fn main() {
    let mut entries = lis::LisBuilder::new()
        .recursive(true)
        .all(true)
        .build(".")
        .list();

    // Sort by size, descending (reverse=true)
    sort_entries(&mut entries, SortBy::Size, true);

    for entry in entries.iter().take(10) {
        println!(
            "{:<10} {:>10} {}",
            entry.mode, entry.size, entry.name
        );
    }
}
```
