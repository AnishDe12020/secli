use crate::core::commands::{add::add, get::get};
use anyhow::Result;
use clap::{Command as ClapCommand, Values};
use std::{path::PathBuf, process::exit};

pub enum Command {
    Add(Option<Values<'static>>),
    Get(Option<Values<'static>>),
}

pub struct App {
    db_path: PathBuf,
}

pub fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join("config.json");
    return config_path;
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            db_path: PathBuf::from("test.db"),
        }
    }

    fn build(&self) -> ClapCommand {
        ClapCommand::new("secli")
            .about("A CLI in Rust to store secrets")
            .subcommand(ClapCommand::new("add").about("Add a secret"))
            .subcommand(ClapCommand::new("get").about("Get a secret"))
    }

    fn get_command(&self) -> Command {
        let options = self.build();

        let matches = Box::leak(options.clone().get_matches().into());

        match matches.subcommand() {
            Some(("add", _)) => Command::Add(None),
            Some(("get", _)) => Command::Get(None),
            _ => {
                println!("No command specified");
                exit(1);
            }
        }
    }

    pub fn run(self) -> Result<()> {
        let command = self.get_command();

        match command {
            Command::Add(args) => add(self, args),
            Command::Get(args) => get(self, args),
        }
    }
}
