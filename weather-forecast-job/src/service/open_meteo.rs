use anyhow::Result;
use chrono::{Duration, Local, NaiveDate};
use reqwest;
use serde_json;

use super::{HistoricalWeather, HistoricalWeatherDailyAggregation};

const DATE_FORMAT: &str = "%Y-%m-%d";

pub(crate) fn get_historical_weather(start_date: Option<String>) -> Result<Option<HistoricalWeatherDailyAggregation>> {
    let start_date = if let Some(start_date) = start_date {
        start_date
    } else {
        "1940-01-01".to_string()
    };
    let start_date = NaiveDate::parse_from_str(&start_date, DATE_FORMAT)? + Duration::days(1);
    // See: https://open-meteo.com/en/docs/historical-weather-api
    // Get data up to yesterday because sometimes `today` is out of allowed range
    let yesterday = Local::now().date_naive() - Duration::days(1);
    if yesterday <= start_date {
        return Ok(None);
    }
    let url = format!(
        "https://archive-api.open-meteo.com/v1/archive?latitude=35.6895&longitude=139.6917\
        &start_date={start_date}\
        &end_date={yesterday}\
        &daily=temperature_2m_max,temperature_2m_min\
        &timezone=Asia%2FTokyo"
    );
    let text = reqwest::blocking::get(url)?.text()?;
    let daily_aggregation = serde_json::from_str::<HistoricalWeather>(&text)?.daily;
    return Ok(Some(daily_aggregation));
}
