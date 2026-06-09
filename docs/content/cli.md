---
title: "CLI Usage"
date: 2026-06-09
draft: false
---

# 💻 lis CLI Tool

The `lis` command-line interface provides a powerful way to list directory entries.

## 🚀 Usage

```bash
lis [OPTIONS] [PATH]
```

## ⚙️ Options

| Option | Description |
| :--- | :--- |
| `-l`, `--long` | Display in long format with details (permissions, owner, size, etc.). |
| `--sort <SORT>` | Sort entries by `name` (default), `time`, `size`, or `extension`. |
| `--reverse` | Reverse the sort order. |
| `-a`, `--all` | List all entries, including hidden files, while respecting `.gitignore`. |
| `-A`, `--whole-all` | List all entries, including hidden files, ignoring `.gitignore`. |
| `--icon` | Display icons for each entry (requires Nerd Font). |
| `--format <FORMAT>` | Specify output format: `plain` (default), `csv`, `json`, or `yaml`. |
| `-R`, `--recursive` | List entries recursively. |

## 🌟 Examples

### Long format with icons
Show detailed information along with file icons.
```bash
lis -l --icon
```

### Recursive listing in JSON
Export the entire directory structure as JSON.
```bash
lis -R --format json
```

### Sort by size
List all entries (including hidden ones) sorted by size in descending order.
```bash
lis -a --sort size --reverse
```

### Ignoring .gitignore
List all files, even those normally ignored by Git.
```bash
lis -A
```
