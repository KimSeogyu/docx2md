# dm2xcod

[![PyPI](https://img.shields.io/pypi/v/dm2xcod.svg)](https://pypi.org/project/dm2xcod/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A high-performance DOCX to Markdown converter written in Rust, with Python bindings.

## Features

- **Fast & Efficient**: Written in Rust for maximum performance.
- **Rich Formatting**: Preserves bold, italic, underline, strikethrough, and more.
  - Uses HTML tags (`<strong>`, `<em>`) for better cross-parser compatibility.
- **Structure Preservation**: Handles heading hierarchy, lists (ordered/unordered), and tables.
- **Image Support**: Extracts and embeds images.
- **Cross-Platform**: Pre-built wheels for macOS (Intel/Apple Silicon), Windows, and Linux.
- **Simple API**: Native Python bindings provided via PyO3.

## Requirements

- **Rust**: 1.75+ (for building from source)
- **Python**: 3.12+ (Universal ABI3 support - works with 3.12, 3.13, 3.14+, etc.)

## Installation

### Python

Install via pip:

```bash
pip install dm2xcod
```

### CLI

Install via cargo:

```bash
cargo install dm2xcod
```

### Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
dm2xcod = "0.3"
```

## Usage

### CLI

```bash
dm2xcod input.docx output.md
```

### Python

```python
import dm2xcod

# Basic conversion
markdown = dm2xcod.convert_docx("document.docx")
print(markdown)

# With options (if applicable in future versions)
# markdown = dm2xcod.convert_docx("document.docx", image_dir="images")
```

### Rust

```rust
use dm2xcod::{DocxToMarkdown, ConvertOptions};

fn main() -> anyhow::Result<()> {
    let converter = DocxToMarkdown::new(ConvertOptions::default());
    let markdown = converter.convert("document.docx")?;
    println!("{}", markdown);
    Ok(())
}
```

### Rust (Advanced: Custom Extractor/Renderer Injection)

`DocxToMarkdown::with_components(...)` lets you replace the default DOCX extractor and Markdown renderer.

```rust
use dm2xcod::adapters::docx::AstExtractor;
use dm2xcod::converter::ConversionContext;
use dm2xcod::core::ast::{BlockNode, DocumentAst};
use dm2xcod::render::Renderer;
use dm2xcod::{ConvertOptions, DocxToMarkdown, Result};
use rs_docx::document::BodyContent;

#[derive(Debug, Default, Clone, Copy)]
struct MyExtractor;

impl AstExtractor for MyExtractor {
    fn extract<'a>(
        &self,
        _body: &[BodyContent<'a>],
        _context: &mut ConversionContext<'a>,
    ) -> Result<DocumentAst> {
        Ok(DocumentAst {
            blocks: vec![BlockNode::Paragraph("custom pipeline".to_string())],
            references: Default::default(),
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct MyRenderer;

impl Renderer for MyRenderer {
    fn render(&self, document: &DocumentAst) -> Result<String> {
        Ok(format!("blocks={}", document.blocks.len()))
    }
}

fn main() -> Result<()> {
    let converter = DocxToMarkdown::with_components(
        ConvertOptions::default(),
        MyExtractor,
        MyRenderer,
    );
    let output = converter.convert("document.docx")?;
    println!("{}", output);
    Ok(())
}
```

## Development

### Build from Source

```bash
# Build Rust library/CLI
cargo build --release

# Development with Python
pip install maturin
maturin develop --features python
```

### Performance Benchmark

```bash
# Uses tests/aaa, 3 iterations, up to 5 DOCX files by default
./scripts/run_perf_benchmark.sh

# Custom input_dir / iterations / max_files
./scripts/run_perf_benchmark.sh ./samples 5 10
```

### Performance Threshold Gate

```bash
# Fails if avg_ms exceeds threshold
./scripts/check_perf_threshold.sh ./output_tests/perf/latest.json 15.0
```

### Release Notes

```bash
# Auto-detect previous tag to HEAD
./scripts/generate_release_notes.sh

# Explicit range and output file
./scripts/generate_release_notes.sh v0.3.9 v0.3.10 ./output_tests/release_notes.md
```

### API Stability

See `docs/API_POLICY.md`.

## License

MIT
