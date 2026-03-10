# md-prune

Extract bold (`**...**`) and highlighted (`==...==`) text from Markdown files,
producing a pruned document that keeps only the key passages you've marked.

## Use Cases

- **Study note distillation**: Extract key takeaways from textbook notes
  you've bolded/highlighted while reading
- **Document summarization**: Reduce long Markdown documents to their
  most important points

## Installation

### From source

Requires [Rust](https://www.rust-lang.org/tools/install) 1.70+.

```bash
git clone https://github.com/your-org/md-prune.git
cd md-prune
cargo build --release
```

The binary will be at `target/release/md-prune`.

#### Platform notes

- **Linux**: No additional dependencies. Tested on Ubuntu 22.04+, Fedora 38+.
- **macOS**: No additional dependencies. Tested on macOS 13+ (Intel and Apple Silicon).
- **Windows**: No additional dependencies. Tested on Windows 10+ with MSVC toolchain.
  Use `cargo build --release` from a Developer Command Prompt or PowerShell.

## Usage

```
md-prune [OPTIONS] <INPUT>
```

### Basic examples

```bash
# Print pruned output to stdout
md-prune notes.md

# Save to a file
md-prune notes.md -o pruned.md

# Pipe to another tool
md-prune notes.md | less
```

### Filter modes

By default, both bold and highlighted text are extracted.

```bash
# Only bold text
md-prune --bold-only notes.md

# Only highlighted text
md-prune --highlight-only notes.md
```

### Excluding content types

By default, headings, images, tables, code blocks, math blocks, and callouts
are all kept in the output. Use `--no-*` flags to exclude them:

```bash
# Minimal output: just the bold/highlighted text
md-prune --no-headings --no-images --no-tables --no-code-blocks --no-math --no-callouts notes.md

# Keep headings but drop everything else
md-prune --no-images --no-tables --no-code-blocks --no-math --no-callouts notes.md
```

### Extraction behavior

```bash
# Keep entire paragraph if any part is bold/highlighted
md-prune --keep-paragraph notes.md

# Keep all headings, even sections with no bold/highlighted text
md-prune --keep-all-headings notes.md

# Strip the ** and == markers from the output
md-prune --strip-markers notes.md
```

### All options

| Flag | Default | Description |
|---|---|---|
| `-o, --output <FILE>` | stdout | Output file path |
| `--bold-only` | off | Only extract `**bold**` text |
| `--highlight-only` | off | Only extract `==highlighted==` text |
| `--no-headings` | off | Exclude headings |
| `--no-images` | off | Exclude images |
| `--no-tables` | off | Exclude tables (Markdown and HTML) |
| `--no-code-blocks` | off | Exclude fenced code blocks |
| `--no-math` | off | Exclude display math blocks |
| `--no-callouts` | off | Exclude callout blocks |
| `--keep-paragraph` | off | Keep full paragraph if any part is marked |
| `--keep-all-headings` | off | Keep all headings, even empty sections |
| `--strip-markers` | off | Remove `**` and `==` from output |

## Supported Markdown features

- **Bold**: `**text**`
- **Highlight**: `==text==` (Obsidian/extended Markdown)
- **Headings**: ATX style (`# H1` through `###### H6`)
- **Images**: Standard `![alt](url)` and Obsidian `![[file]]`
- **Code blocks**: Fenced with 3+ backticks or tildes
- **Display math**: `$$...$$` blocks
- **HTML tables**: `<table>...</table>`
- **Markdown tables**: Pipe-delimited with separator rows
- **Callout blocks**: `> [!info]`, `> [!success]`, etc.
- **Blockquotes**: `> text`
- **List items**: Unordered (`-`, `*`, `+`) and ordered (`1.`, `2.`)

## How it works

md-prune uses a two-pass line parser:

1. **Pass 1 (Block Classification)**: A state machine classifies each line
   into block types (heading, code block, paragraph, etc.). Multi-line
   constructs like code fences are tracked to avoid false positives.

2. **Pass 2 (Content Extraction)**: For text blocks (paragraphs, blockquotes,
   list items, callouts), regex patterns find bold and highlight spans.
   Structural blocks are kept or dropped based on CLI flags.

3. **Heading Pruning**: Headings with no extracted content beneath them
   are removed (unless `--keep-all-headings` is set).

## License

MIT
