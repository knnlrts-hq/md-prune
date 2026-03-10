use std::fs;
use std::io::Write;

use anyhow::Context;
use clap::Parser;

use md_prune::cli::Cli;
use md_prune::errors::MdPruneError;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = cli.to_config();

    let input_path = &cli.input;
    if !input_path.exists() {
        return Err(MdPruneError::FileNotFound {
            path: input_path.clone(),
        }
        .into());
    }

    let input = fs::read_to_string(input_path).map_err(|e| MdPruneError::ReadError {
        path: input_path.clone(),
        source: e,
    })?;

    // Handle UTF-8 BOM
    let input = input.strip_prefix('\u{feff}').unwrap_or(&input);

    let output = md_prune::pruner::prune(input, &config);

    if output.is_empty() {
        eprintln!(
            "md-prune: warning: no bold or highlighted text found in {:?}",
            input_path
        );
    }

    match &cli.output {
        Some(output_path) => {
            fs::write(output_path, &output).map_err(|e| MdPruneError::WriteError {
                path: output_path.clone(),
                source: e,
            })?;
            eprintln!("md-prune: wrote output to {:?}", output_path);
        }
        None => {
            std::io::stdout()
                .write_all(output.as_bytes())
                .context("failed to write to stdout")?;
        }
    }

    Ok(())
}
