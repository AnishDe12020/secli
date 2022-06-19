use crate::cli::app::App;
use crate::core::db::{get_secret, update_secret, Secret};
use anyhow::Result;
use clap::Values;
use colored::Colorize;
use inquire::Text;
use rusqlite::Connection;

pub fn update(app: App, args: Option<Values>) -> Result<()> {
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

    match get_secret(&conn, &secret.name) {
        Ok(_) => match update_secret(&conn, &secret) {
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
        },
        Err(_) => {
            println!(
                "{}",
                format!("Secret with name {} doesn't exist", &secret.name.cyan()).red()
            );
        }
    }

    Ok(())
}
