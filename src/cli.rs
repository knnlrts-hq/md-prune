use std::path::PathBuf;

use clap::Parser;

use crate::types::{Config, FilterMode};

/// Extract bold and highlighted text from Markdown files.
///
/// Parses a Markdown file and outputs a pruned version containing only
/// bold (**...**) and/or highlighted (==...==) text, along with optionally
/// retained structural elements like headings, images, tables, and more.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Path to the input Markdown file
    pub input: PathBuf,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    // -- Filter mode (mutually exclusive) --
    /// Only extract bold (**...**) text
    #[arg(long, conflicts_with = "highlight_only")]
    pub bold_only: bool,

    /// Only extract highlighted (==...==) text
    #[arg(long, conflicts_with = "bold_only")]
    pub highlight_only: bool,

    // -- Content toggles (default: keep) --
    /// Exclude headings from output
    #[arg(long)]
    pub no_headings: bool,

    /// Exclude images from output
    #[arg(long)]
    pub no_images: bool,

    /// Exclude tables (Markdown and HTML) from output
    #[arg(long)]
    pub no_tables: bool,

    /// Exclude fenced code blocks from output
    #[arg(long)]
    pub no_code_blocks: bool,

    /// Exclude display math blocks ($$...$$) from output
    #[arg(long)]
    pub no_math: bool,

    /// Exclude callout blocks (> [!type]) from output
    #[arg(long)]
    pub no_callouts: bool,

    // -- Extraction behavior --
    /// Keep entire paragraph when any part is bold/highlighted
    #[arg(long)]
    pub keep_paragraph: bool,

    /// Keep all headings, even for empty sections
    #[arg(long)]
    pub keep_all_headings: bool,

    /// Remove **bold** and ==highlight== markers from output text
    #[arg(long)]
    pub strip_markers: bool,
}

impl Cli {
    /// Convert parsed CLI arguments into a [`Config`] for the pruning engine.
    ///
    /// The `--no-*` flags are inverted: `--no-headings` sets `keep_headings = false`.
    pub fn to_config(&self) -> Config {
        let filter_mode = if self.bold_only {
            FilterMode::BoldOnly
        } else if self.highlight_only {
            FilterMode::HighlightOnly
        } else {
            FilterMode::Both
        };

        Config {
            filter_mode,
            keep_headings: !self.no_headings,
            keep_images: !self.no_images,
            keep_tables: !self.no_tables,
            keep_code_blocks: !self.no_code_blocks,
            keep_math: !self.no_math,
            keep_callouts: !self.no_callouts,
            keep_paragraph: self.keep_paragraph,
            keep_all_headings: self.keep_all_headings,
            strip_markers: self.strip_markers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_invocation() {
        let args =
            Cli::try_parse_from(["md-prune", "input.md"]).expect("should parse minimal args");
        assert_eq!(args.input.to_str().unwrap(), "input.md");
        assert!(args.output.is_none());
        assert!(!args.bold_only);
        assert!(!args.highlight_only);
        assert!(!args.no_headings);
        assert!(!args.no_images);
        assert!(!args.no_tables);
        assert!(!args.no_code_blocks);
        assert!(!args.no_math);
        assert!(!args.no_callouts);
        assert!(!args.keep_paragraph);
        assert!(!args.keep_all_headings);
        assert!(!args.strip_markers);
    }

    #[test]
    fn output_flag() {
        let args = Cli::try_parse_from(["md-prune", "-o", "out.md", "input.md"])
            .expect("should parse -o flag");
        assert_eq!(args.output.unwrap().to_str().unwrap(), "out.md");
    }

    #[test]
    fn all_no_flags() {
        let args = Cli::try_parse_from([
            "md-prune",
            "--no-headings",
            "--no-images",
            "--no-tables",
            "--no-code-blocks",
            "--no-math",
            "--no-callouts",
            "input.md",
        ])
        .expect("should parse all --no-* flags");
        assert!(args.no_headings);
        assert!(args.no_images);
        assert!(args.no_tables);
        assert!(args.no_code_blocks);
        assert!(args.no_math);
        assert!(args.no_callouts);
    }

    #[test]
    fn bold_only_and_highlight_only_conflict() {
        let result =
            Cli::try_parse_from(["md-prune", "--bold-only", "--highlight-only", "input.md"]);
        assert!(
            result.is_err(),
            "bold-only and highlight-only should conflict"
        );
    }

    #[test]
    fn behavior_flags() {
        let args = Cli::try_parse_from([
            "md-prune",
            "--keep-paragraph",
            "--keep-all-headings",
            "--strip-markers",
            "input.md",
        ])
        .expect("should parse behavior flags");
        assert!(args.keep_paragraph);
        assert!(args.keep_all_headings);
        assert!(args.strip_markers);
    }

    #[test]
    fn to_config_defaults() {
        let args = Cli::try_parse_from(["md-prune", "input.md"]).unwrap();
        let config = args.to_config();
        assert_eq!(config.filter_mode, FilterMode::Both);
        assert!(config.keep_headings);
        assert!(!config.strip_markers);
    }

    #[test]
    fn to_config_bold_only() {
        let args = Cli::try_parse_from(["md-prune", "--bold-only", "input.md"]).unwrap();
        let config = args.to_config();
        assert_eq!(config.filter_mode, FilterMode::BoldOnly);
    }

    #[test]
    fn to_config_no_flags_invert() {
        let args =
            Cli::try_parse_from(["md-prune", "--no-headings", "--no-tables", "input.md"]).unwrap();
        let config = args.to_config();
        assert!(!config.keep_headings);
        assert!(!config.keep_tables);
        assert!(config.keep_images);
    }

    #[test]
    fn missing_input_is_error() {
        let result = Cli::try_parse_from(["md-prune"]);
        assert!(result.is_err(), "should require input file");
    }
}
