use anyhow::Result;
use cli::app::App;
use rusqlite::Connection;

pub mod cli;
pub mod core;

fn main() -> Result<()> {
    let conn = Connection::open("secli.db").unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS secrets (
            name TEXT PRIMARY KEY,
            value TEXT
        )
        ",
        [],
    )
    .unwrap();

    App::new().run()
}
