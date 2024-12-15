mod initiative_tracker;
mod initiative_item;
mod cli;

use clap::Parser;
use crate::{
    initiative_item::InitiativeItem,
    cli::CLI,
};
use crate::initiative_tracker::InitiativeTracker;

fn main() {
    let cli: CLI = CLI::parse();

    let tracker: Vec<InitiativeItem<String>> = match cli.get_encounter() {
        Ok(tracker) => tracker,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut runner: InitiativeTracker = InitiativeTracker::new(tracker);
    runner.run();
}

