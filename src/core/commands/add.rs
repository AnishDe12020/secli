use crate::cli::app::App;
use crate::core::db::{insert_secret, Secret};
use anyhow::Result;
use clap::Values;
use inquire::Text;
use rusqlite::Connection;

pub fn add(app: App, _args: Option<Values>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let name = Text::new("Enter the name/key for this secret: ")
        .prompt()
        .unwrap();

    let value = Text::new("Enter the value for this secret: ")
        .prompt()
        .unwrap();

    let secret = Secret { name, value };

    insert_secret(&conn, &secret)?;

    Ok(())
}
