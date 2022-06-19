use crate::core::{
    commands::{add::add, delete::delete, get::get, list::list, update::update},
    db::get_db_path,
};
use anyhow::Result;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, Arg, Command as ClapCommand,
    Values,
};
use std::{path::PathBuf, process::exit};

pub enum Command {
    Add(Option<Values<'static>>),
    Get(Option<Values<'static>>),
    List(Option<Values<'static>>),
    Update(Option<Values<'static>>),
    Delete(Option<Values<'static>>),
}

pub struct App {
    pub db_path: PathBuf,
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
            db_path: get_db_path(),
        }
    }

    fn build(&self) -> ClapCommand {
        ClapCommand::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .subcommand(
                ClapCommand::new("add")
                    .about("Add a secret")
                    .arg(Arg::new("name").takes_value(true).index(1))
                    .alias("new")
                    .alias("insert")
                    .alias("create"),
            )
            .subcommand(
                ClapCommand::new("get")
                    .about("Get a secret")
                    .arg(Arg::new("name").takes_value(true).index(1))
                    .alias("read")
                    .alias("show"),
            )
            .subcommand(
                ClapCommand::new("list")
                    .about("List all secrets")
                    .alias("ls"),
            )
            .subcommand(
                ClapCommand::new("update")
                    .about("Upate a secret")
                    .arg(Arg::new("name").takes_value(true).index(1)),
            )
            .subcommand(
                ClapCommand::new("delete")
                    .about("Delete a secret")
                    .arg(Arg::new("name").takes_value(true).index(1))
                    .alias("remove"),
            )
    }

    fn get_command(&self) -> Command {
        let options = self.build();

        let matches = Box::leak(options.clone().get_matches().into());

        match matches.subcommand() {
            Some(("add", sub)) => Command::Add(sub.values_of("name")),
            Some(("get", sub)) => Command::Get(sub.values_of("name")),
            Some(("list", _)) => Command::List(None),
            Some(("update", sub)) => Command::Update(sub.values_of("name")),
            Some(("delete", sub)) => Command::Delete(sub.values_of("name")),
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
            Command::Update(args) => update(self, args),
            Command::Delete(args) => delete(self, args),
        }
    }
}
