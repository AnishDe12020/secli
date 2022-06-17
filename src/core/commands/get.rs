use crate::{cli::app::App, core::db::get_secret};
use anyhow::Result;
use clap::Values;
use inquire::Text;
use rusqlite::Connection;

pub fn get(app: App, args: Option<Values<'_>>) -> Result<()> {
    let conn = Connection::open(app.db_path).unwrap();

    let mut args = args.unwrap_or_default();
    let name: String;

    if args.len() == 0 {
        name = Text::new("Name:").prompt().unwrap();
    } else {
        name = args.next().unwrap().to_string();
    }

    let value = get_secret(&conn, &name)?;

    println!("{}", value);

    Ok(())
}
