use crate::cli::app::App;
use anyhow::Result;
use clap::Values;

pub fn get(app: App, _args: Option<Values>) -> Result<()> {
    println!("Get a secret");
    Ok(())
}
