use clap::Parser;

use md_prune::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let _config = cli.to_config();

    eprintln!(
        "md-prune: would process {:?} (not yet implemented)",
        cli.input
    );
    Ok(())
}
