use crate::arguments::{FlashCardCli, FlashCardCommand};
use clap::Parser;
use std::fs;

mod arguments;

fn main() -> anyhow::Result<()> {
    let arguments = FlashCardCli::parse();
    match arguments.command() {
        FlashCardCommand::Convert { card, csv } => {
            let file = fs::read_to_string(card)?;
        }
    }
    Ok(())
}
