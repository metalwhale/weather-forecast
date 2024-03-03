use std::env;

use anyhow::Result;

use crate::repository::Repository;

mod command;
mod repository;
mod schema;
mod service;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let repo = Repository::new()?;
    if args.len() >= 2 {
        match args[1].as_str() {
            "collect-daily-records" => command::historical_weather::collect_daily_records(repo)?,
            _ => {}
        }
    }
    Ok(())
}
