use crate::arguments::{FlashCardCli, FlashCardCommand};
use clap::Parser;

mod arguments;

fn main() {
    let arguments = FlashCardCli::parse();
    match arguments.command() {
        FlashCardCommand::Convert { card, csv } => {}
    }
}
