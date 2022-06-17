use crate::{cli::app::App, core::db::get_secrets};
use anyhow::Result;
use clap::Values;
use colored::Colorize;
use rusqlite::Connection;

pub fn list(app: App, _args: Option<Values<'_>>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let secrets = get_secrets(&conn).unwrap();

    let num_secrets = secrets.len();

    if num_secrets == 0 {
        println!(
            "No secrets found. Add one by running {}{}{}",
            "`".cyan(),
            "secli add".green(),
            "`".cyan()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!(
            "{} {} found",
            secrets.len(),
            if secrets.len() == 1 {
                "secret"
            } else {
                "secrets"
            }
        )
        .green()
    );

    for secret in secrets {
        println!("{}: {}", secret.name, secret.value);
    }

    Ok(())
}
