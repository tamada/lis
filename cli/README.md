# 💻 lis CLI Tool

`lis` is a command-line interface for listing directory entries with advanced features.

## 🛠️ Installation

```bash
cargo install --path .
```

## 🚀 Usage

```bash
lis [OPTIONS] [PATH]
```

### ⚙️ Options

- `-l`, `--long`: Display in long format with details.
- `--sort <SORT>`: Sort entries by `name` (default), `time`, `size`, or `extension`.
- `--reverse`: Reverse the sort order.
- `-a`, `--all`: List all entries, respecting `.gitignore`.
- `-A`, `--whole-all`: List all entries, ignoring `.gitignore`.
- `--icon`: Display icons for each entry.
- `--format <FORMAT>`: Specify output format: `plain` (default), `csv`, `json`, or `yaml`.
- `-R`, `--recursive`: List entries recursively.

### 🌟 Examples

- **List current directory in long format with icons:**
  ```bash
  lis -l --icon
  ```

- **List current directory recursively in JSON format:**
  ```bash
  lis -R --format json
  ```

- **List all entries (including hidden) sorted by size:**
  ```bash
  lis -a --sort size
  ```

- **List hidden files ignoring `.gitignore`:**
  ```bash
  lis -A
  ```

## 📖 Specifications

The detailed specification of `lis` can be found in [.github/assets/spec.md](../.github/assets/spec.md).
