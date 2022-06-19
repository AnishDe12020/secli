use crate::cli::app::App;
use crate::core::db::{insert_secret, Secret};
use anyhow::Result;
use clap::Values;
use colored::Colorize;
use inquire::Text;
use rusqlite::Connection;

pub fn add(app: App, args: Option<Values>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let mut args = args.unwrap_or_default();

    let name: String = if args.len() == 0 {
        match Text::new("Enter the name/key for this secret:").prompt() {
            Ok(name) => name,
            Err(_) => return Ok(()),
        }
    } else {
        args.next().unwrap().to_string()
    };

    let value = match Text::new("Enter the value for this secret:").prompt() {
        Ok(value) => value,
        Err(_) => return Ok(()),
    };

    let secret = Secret { name, value };

    match insert_secret(&conn, &secret) {
        Ok(_) => {
            println!(
                "{}",
                format!(
                    "Secret with name {}{}{} added successfully",
                    "`".yellow(),
                    secret.name.cyan(),
                    "`".yellow()
                )
                .green(),
            );
        }
        Err(err) => {
            println!("{}", err.to_string().red());
            return Ok(());
        }
    }

    Ok(())
}
