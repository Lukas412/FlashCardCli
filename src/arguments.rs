use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub(crate) struct FlashCardCli {
    #[clap(subcommand)]
    command: FlashCardCommand,
}

#[derive(Debug, Subcommand)]
enum FlashCardCommand {
    Convert { card: PathBuf, csv: PathBuf },
}
