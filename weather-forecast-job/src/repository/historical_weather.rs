use std::env;

use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use diesel::{pg::Pg, prelude::*, result::Error::NotFound, sql_types::*};

use super::Repository;
use crate::{schema::historical_weather_daily_records, service::HistoricalWeatherDailyAggregation};

impl Repository {
    pub(crate) fn new() -> Result<Self> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("DATABASE_USER")?,
            env::var("DATABASE_PASSWORD")?,
            env::var("DATABASE_HOST")?,
            env::var("DATABASE_PORT").unwrap_or("5432".to_string()),
            env::var("DATABASE_DB")?,
        );
        return Ok(Self {
            connection: PgConnection::establish(&database_url)?,
        });
    }

    pub(crate) fn find_max_date(&mut self) -> Result<Option<String>> {
        return match diesel::sql_query(
            "SELECT date \
            FROM historical_weather_daily_records \
            ORDER BY date DESC LIMIT 1",
        )
        .get_result::<HistoricalWeatherDailyRecordDate>(&mut self.connection)
        {
            Ok(result) => Ok(Some(result.date)),
            Err(NotFound) => Ok(None),
            Err(e) => bail!(e),
        };
    }

    pub(crate) fn insert_daily_records(&mut self, aggregation: HistoricalWeatherDailyAggregation) -> Result<()> {
        if aggregation.time.len() != aggregation.temperature_2m_max.len()
            || aggregation.time.len() != aggregation.temperature_2m_min.len()
            || aggregation.temperature_2m_max.len() != aggregation.temperature_2m_min.len()
        {
            bail!("Daily aggregation variables do not have the same length.");
        }
        let mut records = vec![];
        for i in 0..aggregation.time.len() {
            records.push(InsertHistoricalWeatherDailyRecord {
                date: aggregation.time[i].clone(), // Will be truncated to string with length of 10, formatted as "YYYY-MM-DD"
                temperature_2m_max: aggregation.temperature_2m_max[i],
                temperature_2m_min: aggregation.temperature_2m_min[i],
                created_at: Local::now(),
                updated_at: Local::now(),
            })
        }
        // Resolve the "number of parameters must be between 0 and 65535" error by chunking
        // "number of parameters" = number of records * number of columns (?)
        for chunk in records.chunks(10000) {
            diesel::insert_into(historical_weather_daily_records::table)
                .values(chunk)
                .returning(InsertHistoricalWeatherDailyRecord::as_returning())
                .get_result(&mut self.connection)?;
        }
        Ok(())
    }
}

#[derive(QueryableByName)]
struct HistoricalWeatherDailyRecordDate {
    #[diesel(sql_type = Text)]
    date: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = historical_weather_daily_records)]
#[diesel(check_for_backend(Pg))]
struct InsertHistoricalWeatherDailyRecord {
    date: String,
    temperature_2m_max: Option<f32>,
    temperature_2m_min: Option<f32>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}
