use std::{fs::create_dir_all, path::PathBuf};

use anyhow::{bail, Result};
use clap::crate_name;
use colored::Colorize;
use rusqlite::Connection;

pub fn get_data_path() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap();
    let secli_dir = data_dir.join(format!("{}", crate_name!()));
    secli_dir
}

pub fn get_db_path() -> PathBuf {
    let db_path = get_data_path().join("secli.db");
    db_path
}

pub fn create_db() {
    let seclir_dir = get_data_path();
    let db_path = seclir_dir.join("secli.db");
    if db_path.exists() {
        return;
    }

    create_dir_all(seclir_dir).unwrap();

    let conn = Connection::open(db_path).unwrap();

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
}

#[derive(Debug)]
pub struct Secret {
    pub name: String,
    pub value: String,
}

pub fn insert_secret(conn: &Connection, secret: &Secret) -> Result<()> {
    conn.execute(
        "INSERT INTO secrets (name, value) VALUES (?1, ?2)",
        [&secret.name, &secret.value],
    )?;
    Ok(())
}

pub fn get_secret(conn: &Connection, name: &str) -> Result<String> {
    let mut stmt = conn.prepare("SELECT value FROM secrets WHERE name = ?1")?;
    let mut rows = stmt.query(&[&name])?;

    let row = match rows.next()? {
        Some(row) => row,
        None => bail!("Secret with name {} not found", name.cyan()),
    };

    let value: String = row.get(0)?;
    Ok(value)
}

pub fn get_secrets(conn: &Connection) -> Result<Vec<Secret>> {
    let mut st = conn.prepare("SELECT name, value FROM secrets").unwrap();

    let iter = st.query_map([], |row| {
        Ok(Secret {
            name: row.get(0)?,
            value: row.get(1)?,
        })
    })?;

    let mut secrets = Vec::new();
    for secret in iter {
        secrets.push(secret?);
    }
    Ok(secrets)
}
