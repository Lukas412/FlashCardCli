use crate::arguments::{FlashCardCli, FlashCardCommand};
use clap::Parser;
use flash_card_parser::Topic;
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

mod arguments;

fn main() -> anyhow::Result<()> {
    let arguments = FlashCardCli::parse();
    match arguments.command() {
        FlashCardCommand::Convert { card, json } => {
            let file = fs::read_to_string(card)?;
            let result = Topic::from_str(&file);
            let topic = match result {
                Ok(topic) => topic,
                Err(error) => panic!("{:?}", error),
            };
            println!("{}", &topic);
            let file = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(json)?;
            let buffer = BufWriter::new(file);
            serde_json::to_writer(buffer, &topic)?;
        }
    }
    Ok(())
}
