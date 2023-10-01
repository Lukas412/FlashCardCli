use crate::arguments::{FlashCardCli, FlashCardCommand};
use clap::Parser;
use flash_card_parser::Topic;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod arguments;

fn main() -> anyhow::Result<()> {
    let arguments = FlashCardCli::parse();
    match arguments.command() {
        FlashCardCommand::ConvertSingle {
            path,
            output: Some(output),
        } => convert_file_to_output(path, output),
        FlashCardCommand::ConvertSingle { path, output: None } => convert_file(path),
        FlashCardCommand::ConvertAll { path } => convert_all(path),
    }
}

fn convert_all(path: impl AsRef<Path>) -> anyhow::Result<()> {
    for file in fs::read_dir(path)? {
        let file = file?;
        convert_file(file.path())?
    }
    Ok(())
}

fn convert_file(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let output = path.as_ref().with_extension("json");
    convert_file_to_output(path, output)
}

fn convert_file_to_output(path: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = fs::read_to_string(path)?;
    let topic = match Topic::from_str(file.as_str()) {
        Ok(topic) => topic,
        Err(error) => panic!("{:?}", error),
    };
    println!("{}", &topic);

    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;
    let buffer = BufWriter::new(file);
    serde_json::to_writer(buffer, &topic)?;
    Ok(())
}
