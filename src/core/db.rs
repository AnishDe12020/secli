use anyhow::Result;
use rusqlite::Connection;

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
    let row = rows.next()?.unwrap();
    let value: String = row.get(0)?;
    Ok(value)
}

pub fn get_secrets(conn: &Connection) -> Result<Vec<Secret>> {
    let mut st = conn.prepare("SELECT name, value FROm secrets").unwrap();

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
