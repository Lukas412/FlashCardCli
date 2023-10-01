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
    ConvertSingle {
        #[arg(short, long)]
        path: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    ConvertAll {
        #[arg(short, long)]
        path: PathBuf,
    },
}
