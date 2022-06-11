use anyhow::Result;
use cli::app::App;
use rusqlite::Connection;

pub mod cli;
pub mod core;

#[derive(Debug)]
struct Secret {
    pub name: String,
    pub value: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("secli.db").unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS secli (
            name TEXT PRIMARY KEY,
            value TEXT
        )
        ",
        [],
    )
    .unwrap();

    let sec1 = Secret {
        name: "secret1".to_string(),
        value: "secret1".to_string(),
    };

    conn.execute(
        "
        INSERT INTO secli (name, value) VALUES (?1, ?2)",
        [&sec1.name, &sec1.value],
    )
    .unwrap();

    let mut st = conn.prepare("SELECT name, value FROM secli").unwrap();
    let secret_iter = st
        .query_map([], |row| {
            Ok(Secret {
                name: row.get(0).unwrap(),
                value: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for secret in secret_iter {
        println!("{:?}", secret);
    }

    App::new().run()
}
