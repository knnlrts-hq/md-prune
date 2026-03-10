/// Classifies a block of consecutive lines by their Markdown role.
///
/// The parser (pass 1) assigns one of these kinds to every contiguous
/// group of lines in the input file. Later stages decide what to keep
/// or discard based on the kind and the user's CLI flags.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockKind {
    /// ATX heading (`# … ######`). `level` is 1–6.
    Heading { level: u8 },
    /// Fenced code block (` ``` ` or `~~~`).
    /// Includes the opening/closing fence lines.
    CodeBlock,
    /// Display math block (`$$ … $$`).
    /// Includes the opening/closing `$$` lines.
    DisplayMath,
    /// Callout block (`> [!type] …`). All `>` prefixed continuation
    /// lines belong to the same block.
    Callout,
    /// Plain blockquote (`> …`) that is *not* a callout.
    Blockquote,
    /// HTML table (`<table> … </table>`).
    HtmlTable,
    /// Pipe-delimited Markdown table (header + separator + data rows).
    MarkdownTable,
    /// Image line — Obsidian (`![[…]]`) or standard (`![…](…)`).
    Image,
    /// List item (unordered `-`/`*`/`+` or ordered `1.`).
    /// Each item is its own block; nested items are separate blocks.
    ListItem,
    /// Any line that doesn't match another kind — the primary
    /// extraction target for bold/highlight filtering.
    Paragraph,
    /// Blank line(s) between blocks.
    BlankLine,
}

/// A contiguous group of source lines sharing the same [`BlockKind`].
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub kind: BlockKind,
    pub lines: Vec<String>,
}

/// Controls whether we extract bold, highlighted, or both kinds of spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilterMode {
    /// Keep both `**bold**` and `==highlighted==` text.
    #[default]
    Both,
    /// Keep only `**bold**` text.
    BoldOnly,
    /// Keep only `==highlighted==` text.
    HighlightOnly,
}

/// Runtime configuration derived from CLI flags.
///
/// All `keep_*` fields default to `true` (keep the content).
/// The `--no-*` CLI flags flip them to `false`.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub filter_mode: FilterMode,
    pub keep_headings: bool,
    pub keep_images: bool,
    pub keep_tables: bool,
    pub keep_code_blocks: bool,
    pub keep_math: bool,
    pub keep_callouts: bool,
    pub keep_paragraph: bool,
    pub keep_all_headings: bool,
    pub strip_markers: bool,
}

// Rust note for Python devs: `Default` is like __init__ with no args.
// The `impl Default` block below defines what you get from Config::default().
impl Default for Config {
    fn default() -> Self {
        Self {
            filter_mode: FilterMode::default(),
            keep_headings: true,
            keep_images: true,
            keep_tables: true,
            keep_code_blocks: true,
            keep_math: true,
            keep_callouts: true,
            keep_paragraph: false,
            keep_all_headings: false,
            strip_markers: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_kind_is_debug_clone_eq() {
        let a = BlockKind::Heading { level: 1 };
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(format!("{:?}", a), "Heading { level: 1 }");
    }

    #[test]
    fn block_stores_lines() {
        let block = Block {
            kind: BlockKind::Paragraph,
            lines: vec!["hello **world**".to_string()],
        };
        assert_eq!(block.lines.len(), 1);
        assert_eq!(block.kind, BlockKind::Paragraph);
    }

    #[test]
    fn filter_mode_defaults_to_both() {
        let mode = FilterMode::default();
        assert_eq!(mode, FilterMode::Both);
    }

    #[test]
    fn config_defaults() {
        let config = Config::default();
        assert!(config.keep_headings);
        assert!(config.keep_images);
        assert!(config.keep_tables);
        assert!(config.keep_code_blocks);
        assert!(config.keep_math);
        assert!(config.keep_callouts);
        assert!(!config.keep_paragraph);
        assert!(!config.keep_all_headings);
        assert!(!config.strip_markers);
        assert_eq!(config.filter_mode, FilterMode::Both);
    }
}
