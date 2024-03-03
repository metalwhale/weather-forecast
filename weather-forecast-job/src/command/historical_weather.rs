use anyhow::Result;

use crate::{repository::Repository, service};

pub(crate) fn collect_daily_records(mut repo: Repository) -> Result<()> {
    let start_date = repo.find_max_date()?;
    let aggregation = service::open_meteo::get_historical_weather(start_date)?;
    if let Some(aggregation) = aggregation {
        repo.insert_daily_records(aggregation)?;
    }
    Ok(())
}
