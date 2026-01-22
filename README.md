# docx2md

[![Crates.io](https://img.shields.io/crates/v/docx2md.svg)](https://crates.io/crates/docx2md)
[![PyPI](https://img.shields.io/pypi/v/docx2md.svg)](https://pypi.org/project/docx2md/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/KimSeogyu/docx2md/actions/workflows/release.yml/badge.svg)](https://github.com/KimSeogyu/docx2md/actions)

Zero-loss DOCX to Markdown converter, written in Rust with Python bindings.

## Features

- **High Fidelity**: Preserves formatting, tables, and images
- **Heading Support**: Converts DOCX heading levels to Markdown (Heading 1 → `##`, Heading 2 → `###`, etc.)
- **Korean Localization** (optional): Intelligent parsing of Korean headers (e.g., `제1조` → `### 제1조`) with `--lang ko` option
- **Performance**: Fast conversion using Rust
- **Dual Mode**: Use as a CLI tool, Rust library, or Python package

## Requirements

| Component | Version |
|-----------|---------|
| Rust | 1.75+ |
| Python | 3.12+ |

## Installation

### 🐍 Python

```bash
pip install docx2md
```

**Usage:**

```python
import docx2md

markdown = docx2md.convert_docx("document.docx")
print(markdown)
```

### 🦀 Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
docx2md = "0.1"
```

**Usage:**

```rust
use docx2md::{DocxToMarkdown, ConvertOptions};

fn main() {
    let converter = DocxToMarkdown::with_defaults();
    let md = converter.convert("document.docx").unwrap();
    println!("{}", md);
}
```

### 💻 CLI

**Install:**

```bash
cargo install docx2md
```

**Run:**

```bash
docx2md input.docx output.md --lang ko
```

## Building from Source

### Python (with maturin)

```bash
pip install maturin
maturin develop --features python
```

### Rust

```bash
cargo build --release
```

## License

MIT
