use anyhow::Result;
use cli::app::App;

pub mod cli;
pub mod core;

fn main() -> Result<()> {
    App::new().run()
}
