use crate::cli::app::App;
use anyhow::Result;
use clap::Values;

pub fn add(app: App, _args: Option<Values>) -> Result<()> {
    println!("Add a secret");
    Ok(())
}
