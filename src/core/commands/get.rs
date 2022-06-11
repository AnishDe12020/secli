use crate::{cli::app::App, core::db::get_secret};
use anyhow::Result;
use clap::Values;
use inquire::Text;
use rusqlite::Connection;

pub fn get(app: App, _args: Option<Values>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let name = Text::new("Enter the name/key for the secret: ")
        .prompt()
        .unwrap();

    let value = get_secret(&conn, &name)?;

    println!("{}", value);

    Ok(())
}
