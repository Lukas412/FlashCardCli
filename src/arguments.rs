use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub(crate) struct FlashCardCli {
    command: FlashCardCommand,
}

#[derive(Debug, Subcommand)]
enum FlashCardCommand {
    Convert {},
}
