# 🐿️ lis

[![build](https://github.com/tamada/lis/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/lis/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/lis/badge.svg?branch=main)](https://coveralls.io/github/tamada/lis?branch=main)

[![CC-0](https://img.shields.io/badge/License-CC--0-blue.svg)](https://github.com/tamada/lis/blob/main/LICENSE)
[![Version](https://img.shields.io/badge/Version-0.0.10-blue.svg)](https://github.com/tamada/lis/releases/tag/v0.0.10)
[![DOI](https://zenodo.org/badge/1206549651.svg)](https://doi.org/10.5281/zenodo.19938406)

`lis` is a modern, feature-rich directory listing tool written in Rust. It aims to provide a better alternative to `ls` with built-in support for Git status, icons, and multiple output formats.
This product is an example project for learning Rust and the development process of open source software.
It is not intended for production use.

## 🗣️ Overview

`lis` provides a library and a CLI tool for listing directory entries with advanced features like Git status, icons, and multiple output formats.

## 🚀 Features

- **Git Integration**: Shows Git status for files and directories (A, M, D, R, T).
- **Icons**: Beautiful icons for different file types (requires a Nerd Font).
- **Multiple Formats**: Support for Plain, CSV, JSON, and YAML output.
- **Sorting**: Sort by Name, Time, Size, or Extension.
- **Filtering**: Respects `.gitignore` by default.
- **Recursive**: Supports recursive directory listing.
- **Colorized Output**: High-quality colorized output based on `LS_COLORS`.

## 📦 Installation

```bash
cargo install --path .
```

## 💻 Usage (CLI)

### Basic listing
```bash
lis
```

### Long format with icons
```bash
lis -l --icon
```

### Recursive listing including hidden files
```bash
lis -R -a
```

### Export to JSON
```bash
lis --format json > entries.json
```

### Sort by size in reverse order
```bash
lis --sort size -r
```

For more CLI details, see [cli/README.md](cli/README.md).

## 📖 Library Usage

`lis` can also be used as a library in your Rust projects.

```rust
use lis::LisBuilder;

fn main() {
    let lis = LisBuilder::new()
        .all(true)
        .recursive(false)
        .build(".");

    let entries = lis.list();
    for entry in entries {
        println!("{}: {}", entry.name, entry.git_status);
    }
}
```

Check the [examples](examples/) directory for more details.

## ℹ️ About

### 👨‍💼​ Developers 👩‍💼

- Haruaki Tamada ([tamada](https://github.com/tamada))

### 🎃 Logo

![logo](.github/assets/squirrel.png)

This icon is created by [yukyik](https://www.flaticon.com/packs/cute-cartoon-illustration-17593662l) and distributed on [Flaticon](https://www.flaticon.com).

## 📄 License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.
