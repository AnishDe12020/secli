use crate::{
    cli::app::App,
    core::db::{delete_secret, get_secret},
};
use anyhow::Result;
use clap::Values;
use colored::Colorize;
use inquire::Text;
use rusqlite::Connection;

pub fn delete(app: App, args: Option<Values<'_>>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let mut args = args.unwrap_or_default();

    let name: String = if args.len() == 0 {
        Text::new("Name:").prompt().unwrap()
    } else {
        args.next().unwrap().to_string()
    };

    match get_secret(&conn, &name) {
        Ok(_) => match delete_secret(&conn, &name) {
            Ok(_) => {
                println!(
                    "{}",
                    format!(
                        "Secret with name {}{}{} deleted successfully",
                        "`".yellow(),
                        name.cyan(),
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
                format!("Secret with name {} doesn't exist", &name.cyan()).red()
            );
        }
    }

    Ok(())
}
