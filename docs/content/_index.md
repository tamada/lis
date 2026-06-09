---
title: "🐿️ lis -- Modern ls alternative"
date: 2026-06-09
draft: false
---

## 🗣️ Overview

`lis` is a modern, feature-rich directory listing tool written in Rust. It aims to provide a better alternative to `ls` with built-in support for Git status, icons, and multiple output formats.

This product is an example project for learning Rust and the development process of open source software.

`lis` provides a library and a CLI tool for listing directory entries with advanced features like Git status, icons, and multiple output formats.

## 🚀 Quick Start

### Installation

#### 🍺 Homebrew

```bash
brew install tamada/tap/lis
```

#### 💪 Compile myself

```bash
git clone https://github.com/tamada/lis.git
cd lis
cargo build --release
```

### Basic Usage

```bash
lis -l --icon
```

Check the [CLI Guide](cli/) for more details.
