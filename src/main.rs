use crate::arguments::{FlashCardCli, FlashCardCommand};
use clap::Parser;
use flash_card_parser::Topic;
use std::fs;

mod arguments;

fn main() -> anyhow::Result<()> {
    let arguments = FlashCardCli::parse();
    match arguments.command() {
        FlashCardCommand::Convert { card, csv } => {
            let file = fs::read_to_string(card)?;
            let result = Topic::from_str(&file);
            let topic = match result {
                Ok(topic) => topic,
                Err(error) => panic!("{:?}", error),
            };
            println!("{}", &topic);
        }
    }
    Ok(())
}
