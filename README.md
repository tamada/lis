# 🐿️ lis

[![build](https://github.com/tamada/lis/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/lis/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/lis/badge.svg?branch=main)](https://coveralls.io/github/tamada/lis?branch=main)

[![CC-0](https://img.shields.io/badge/License-CC--0-blue.svg)](https://github.com/tamada/lis/blob/main/LICENSE)

Minimal and alternative `ls` implementation in Rust.
This product is an example project for learning Rust and the development process of open source software.
It is not intended for production use.

## 🗣️ Overview

`lis` provides a library and a CLI tool for listing directory entries with advanced features like Git status, icons, and multiple output formats.

## 🚀 Features

- List entries in specified directories.
- Support for sorting (name, time, size, extension) and reverse order.
- Long format with detailed information, including **Git status**.
- Support for hidden files, respecting `.gitignore`.
- Display icons for each entry based on file type and extension.
- Multiple output formats: `plain`, `csv`, `json`, and `yaml`.
- Support for `LS_COLORS` environment variable.

## 📖 Library Usage

Add `lis` to your `Cargo.toml`. Then use it as follows:

```rust
use lis::Lis;
use std::path::PathBuf;

fn main() {
    let entries = Lis::new(PathBuf::from("."))
        .recursive(true)
        .all(true)
        .list();

    for entry in entries {
        println!("{:<10} {}", entry.mode, entry.name);
    }
}
```

Check the [examples](examples/) directory for more details.

## 💻 CLI Usage

See [cli/README.md](cli/README.md) for more details.

## ℹ️ About

### 👨‍💼​ Developers 👩‍💼

- Haruaki Tamada ([tamada](https://github.com/tamada))

### 🎃 Logo

![logo](.github/assets/squirrel.png)

This icon is created by [yukyik](https://www.flaticon.com/packs/cute-cartoon-illustration-17593662l) and distributed on [Flaticon](https://www.flaticon.com).
