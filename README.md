# dm2xcod

[![PyPI](https://img.shields.io/pypi/v/dm2xcod.svg)](https://pypi.org/project/dm2xcod/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

DOCX to Markdown converter. Written in Rust with Python bindings.

## Features

- Converts `.docx` files to Markdown format
- Preserves heading hierarchy, text formatting (bold, italic, underline), tables, and images
- Handles DOCX numbering (ordered/unordered lists)
- Korean heading localization support (`--lang ko`)

## Requirements

- Rust 1.75+
- Python 3.12+ (for Python bindings)

## Installation

### Python

```bash
pip install dm2xcod
```

### Rust

```toml
[dependencies]
dm2xcod = "0.1"
```

### CLI

```bash
cargo install dm2xcod
```

## Usage

### Python

```python
import dm2xcod

markdown = dm2xcod.convert_docx("document.docx")
print(markdown)
```

### Rust

```rust
use dm2xcod::{DocxToMarkdown, ConvertOptions};

let converter = DocxToMarkdown::new(ConvertOptions::default());
let markdown = converter.convert("document.docx").unwrap();
```

### CLI

```bash
dm2xcod input.docx output.md
dm2xcod input.docx --lang ko  # Korean heading localization
```

## Build from Source

```bash
# Rust library
cargo build --release

# Python wheel
pip install maturin
maturin develop --features python
```

## License

MIT
