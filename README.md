# Markdown to text

This Rust library converts Markdown to plain text.

## Usage

Add to your `Cargo.toml`

```toml
[dependencies]
markdown_to_text = '1.0'
```

```rust
let markdown: String = [...];
let plain_text: String = markdown_to_text::convert(&markdown);
```
