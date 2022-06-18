use crate::cli::app::App;
use crate::core::db::{update_secret, Secret};
use anyhow::Result;
use clap::Values;
use colored::Colorize;
use inquire::Text;
use rusqlite::Connection;

pub fn update(app: App, _args: Option<Values>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let name = Text::new("Enter the name/key for this secret:")
        .prompt()
        .unwrap();

    let value = Text::new("Enter the value for this secret:")
        .prompt()
        .unwrap();

    let secret = Secret { name, value };

    match update_secret(&conn, &secret) {
        Ok(_) => {
            println!(
                "{}",
                format!(
                    "Secret with name {}{}{} updated successfully",
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
