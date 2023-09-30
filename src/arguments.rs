use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub(crate) struct FlashCardCli {
    #[clap(subcommand)]
    command: FlashCardCommand,
}

#[derive(Debug, Subcommand)]
enum FlashCardCommand {
    Convert {
        #[arg(short, long)]
        card: PathBuf,
        #[arg(short, long)]
        csv: PathBuf,
    },
}
