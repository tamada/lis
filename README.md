# lis

`lis` is a modern, feature-rich directory listing tool written in Rust. It aims to provide a better alternative to `ls` with built-in support for Git status, icons, and multiple output formats.

![Squirrel](.github/assets/squirrel.png)

## Features

- **Git Integration**: Shows Git status for files and directories (A, M, D, R, T).
- **Icons**: Beautiful icons for different file types (requires a Nerd Font).
- **Multiple Formats**: Support for Plain, CSV, JSON, and YAML output.
- **Sorting**: Sort by Name, Time, Size, or Extension.
- **Filtering**: Respects `.gitignore` by default.
- **Recursive**: Supports recursive directory listing.
- **Colorized Output**: High-quality colorized output based on `LS_COLORS`.

## Installation

```bash
cargo install --path .
```

## Usage

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

## Library Usage

`lis` can also be used as a library in your Rust projects.

```rust
use lis::Lis;
use std::path::PathBuf;

fn main() {
    let lis = Lis::new(PathBuf::from("."))
        .all(true)
        .recursive(false);

    let entries = lis.list();
    for entry in entries {
        println!("{}: {}", entry.name, entry.git_status);
    }
}
```

## CLI Options

```text
Usage: lis [OPTIONS] [PATH]

Arguments:
  [PATH]  Directory to list [default: .]

Options:
  --sort <SORT>      Sort by [default: name] [possible values: name, time, size, extension]
  -r, --reverse          Reverse sort order
  -l, --long             Long format
  -a, --all              All entries, respecting .gitignore
  -A, --whole-all        All entries, ignoring .gitignore
  --icon             Display icons
  --format <FORMAT>  Output format [default: plain] [possible values: plain, csv, json, yaml]
  -R, --recursive        Recursive listing
  -h, --help             Print help
  -V, --version          Print version
```

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.
