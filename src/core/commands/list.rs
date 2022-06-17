use crate::{cli::app::App, core::db::get_secrets};
use anyhow::Result;
use clap::Values;
use rusqlite::Connection;

pub fn list(app: App, _args: Option<Values<'_>>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let secrets = get_secrets(&conn).unwrap();

    println!(
        "{} {} found\n",
        secrets.len(),
        if secrets.len() == 1 {
            "secret"
        } else {
            "secrets"
        }
    );

    for secret in secrets {
        println!("{}: {}", secret.name, secret.value);
    }

    Ok(())
}
