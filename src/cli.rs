use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::{PathBuf};
use clap::{Parser, ValueEnum};
use crate::initiative_item::InitiativeItem;

#[derive(ValueEnum, Clone, Debug)]
enum Command {
    Start,
    Load
}

#[derive(Parser)]
#[command(
author = "Brion Gahl <briongahl@gmail.com>",
version = "0.0.1",
about = "A simple CLI client for D&D initiative tracking"
)]
pub struct CLI {
    #[arg(name = "command", value_enum)]
    command: Command,

    #[arg(name = "FILE", required_if_eq("command", "load"))]
    file_path: Option<PathBuf>
}

impl CLI {
    pub fn get_encounter(&self) -> Result<Vec<InitiativeItem<String>>, Error> {
        match self.command {
            Command::Start => Ok(Vec::new()),
            Command::Load => {
                if let Some(file_path) = &self.file_path {
                    read_file_to_vector(file_path)
                } else {
                    Err(Error::new(ErrorKind::InvalidInput, "No path was provided"))
                }
            }
        }
    }
}

fn read_file_to_vector(path: &PathBuf) -> Result<Vec<InitiativeItem<String>>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut output: Vec<InitiativeItem<String>> = Vec::new(); // TODO: Rewrite this to be more efficient
    for line in reader.lines() {
        let parts: Vec<String> = line?.split("|").map(|part| String::from(part)).collect();

        let name: String = parts.get(0).unwrap().clone();
        let initiative: i32 = parts.get(1).unwrap().parse().unwrap();

        output.push(InitiativeItem::new(name, initiative));
    }
    Ok(output)
}