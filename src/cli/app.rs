use crate::core::{
    commands::{add::add, get::get, list::list},
    db::get_db_path,
};
use anyhow::Result;
use clap::{Arg, Command as ClapCommand, Values};
use std::{path::PathBuf, process::exit};

pub enum Command {
    Add(Option<Values<'static>>),
    Get(Option<Values<'static>>),
    List(Option<Values<'static>>),
}

pub struct App {
    pub db_path: PathBuf,
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
            db_path: PathBuf::from(get_db_path()),
        }
    }

    fn build(&self) -> ClapCommand {
        ClapCommand::new("secli")
            .about("A CLI in Rust to store secrets")
            .subcommand(ClapCommand::new("add").about("Add a secret"))
            .subcommand(
                ClapCommand::new("get")
                    .about("Get a secret")
                    .arg(Arg::new("name").takes_value(true).index(1)),
            )
            .subcommand(
                ClapCommand::new("list")
                    .about("List all secrets")
                    .alias("ls"),
            )
    }

    fn get_command(&self) -> Command {
        let options = self.build();

        let matches = Box::leak(options.clone().get_matches().into());

        match matches.subcommand() {
            Some(("add", _)) => Command::Add(None),
            Some(("get", sub)) => Command::Get(sub.values_of("name")),
            Some(("list", _)) => Command::List(None),
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
            Command::List(args) => list(self, args),
        }
    }
}
