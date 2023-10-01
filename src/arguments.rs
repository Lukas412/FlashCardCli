use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub(crate) struct FlashCardCli {
    #[clap(subcommand)]
    command: FlashCardCommand,
}

impl FlashCardCli {
    pub(crate) fn command(&self) -> &FlashCardCommand {
        &self.command
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum FlashCardCommand {
    Convert {
        #[arg(short, long)]
        card: PathBuf,
        #[arg(short, long)]
        json: PathBuf,
    },
}
