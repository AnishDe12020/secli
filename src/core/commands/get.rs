use crate::{cli::app::App, core::db::get_secret};
use anyhow::Result;
use clap::Values;
use colored::Colorize;
use inquire::Text;
use rusqlite::Connection;

pub fn get(app: App, args: Option<Values<'_>>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let mut args = args.unwrap_or_default();

    let name: String = if args.len() == 0 {
        match Text::new("Name:").prompt() {
            Ok(name) => name,
            Err(_) => return Ok(()),
        }
    } else {
        args.next().unwrap().to_string()
    };

    let value = match get_secret(&conn, &name) {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err.to_string().red());
            return Ok(());
        }
    };

    println!("{}", value);

    Ok(())
}
