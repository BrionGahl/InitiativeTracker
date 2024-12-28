use std::char::ParseCharError;
use std::fs::File;
use std::io;
use std::io::{Error, Write};
use crate::initiative_item::InitiativeItem;

#[derive(Copy, Clone)]
enum State {
    Normal = 0,
    Add = 1,
    Remove = 2,
    Save = 3,
    Quit = 4,
}

pub struct InitiativeTracker {
    tracker: Vec<InitiativeItem<String>>,
    turn: i32,
    round: i32,
    state: State,
}

impl InitiativeTracker {
    pub fn new(vec: Vec<InitiativeItem<String>>) -> InitiativeTracker {
        InitiativeTracker {tracker: vec, state: State::Normal, turn: 0, round: 1}
    }

    fn next_turn(&mut self) {
        if self.turn >= (self.tracker.len() - 1) as i32 {
            self.round += 1;
        };

        self.turn = (self.turn + 1) % self.tracker.len() as i32;
    }

    fn handle_normal(&mut self) {
        display_header("ENCOUNTER");
        println!("Round: {}, Seconds: {}", self.round, (self.round - 1) * 6);

        println!("MARKER | COMBATANT | INITIATIVE | INDEX");
        for (i, item) in self.tracker.iter().enumerate() {
            let marker: &str = if i as i32 == self.turn {"->"} else {""};
            println!("{}\t{}\t{}\t{}", marker, item.name(), item.initiative(), i);
        }
        display_options();

        let output: char = get_option_input().unwrap_or_else(|| 'x');
        match output {
            'n' => self.next_turn(),
            'a' => self.state = State::Add,
            'r' => self.state = State::Remove,
            's' => self.state = State::Save,
            'q' => self.state = State::Quit,
            _ => {}
        }
    }

    fn handle_add(&mut self) {
        display_header("ADD");
        print!("Enter combatant name: ");
        io::stdout().flush().unwrap();

        let name: String = get_string_input();
        print!("Enter combatant initiative: ");
        io::stdout().flush().unwrap();

        let initiative: f32 = match get_string_input().parse::<f32>() {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Couldn't parse initiative... {} ", e);
                self.state = State::Normal;
                return;
            }
        };

        self.add(name, initiative);
        self.state = State::Normal;
    }

    fn handle_remove(&mut self) {
        display_header("REMOVE");
        print!("Enter index to remove: ");
        io::stdout().flush().unwrap();
        let index: usize = match get_string_input().parse::<usize>() {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Couldn't parse index... {}", e);
                self.state = State::Normal;
                return;
            }
        };

        self.tracker.remove(index);
        self.state = State::Normal;
    }

    fn handle_save(&mut self)  -> Result<bool, Error> {
        let mut file: File = File::create("encounter.txt")?;

        for item in self.tracker.iter() {
            file.write(format!("{}|{}\n", item.name(), item.initiative()).as_bytes())?;
        }

        self.state = State::Quit;
        Ok(true)
    }

    fn sort(&mut self) {
        self.tracker.sort_by(|a, b| b.cmp(a));
    }

    fn add(&mut self, name: String, initiative: f32) {
        self.tracker.push(InitiativeItem::new(name, initiative));
        self.sort();
    }

    pub fn run(&mut self) {
        self.sort();

        while self.state as i32 != State::Quit as i32 {
            print!("{}[2J", 27 as char); // Clear Screen
            match self.state {
                State::Normal => {
                    self.handle_normal();
                }
                State::Add => {
                    self.handle_add();
                }
                State::Remove => {
                    self.handle_remove();
                }
                State::Save => {
                    let _ = self.handle_save();
                }
                State::Quit => {}
            }
        }

    }
}

fn display_header(title: &str) {
    println!("------{}------", title);
}

fn display_options() {
    println!("Options: n - Next Turn, a - Add, r - Remove, s - Save, q - Quit");
}

fn get_option_input() -> Option<char> {
    let stdin = io::stdin();

    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();

    let input: Result<char, ParseCharError> = input.trim().parse::<char>();
    match input {
        Ok(i) => Some(i),
        Err(_) => None
    }
}

fn get_string_input() -> String {
    let stdin = io::stdin();

    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}