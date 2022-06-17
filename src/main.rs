use crate::core::db;

use anyhow::Result;
use cli::app::App;

pub mod cli;
pub mod core;

fn main() -> Result<()> {
    db::create_db();

    App::new().run()
}
