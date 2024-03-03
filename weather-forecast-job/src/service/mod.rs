use serde::Deserialize;

pub(crate) mod open_meteo;

#[derive(Deserialize)]
struct HistoricalWeather {
    daily: HistoricalWeatherDailyAggregation,
}

#[derive(Deserialize)]
pub(crate) struct HistoricalWeatherDailyAggregation {
    pub(crate) time: Vec<String>,
    pub(crate) temperature_2m_max: Vec<Option<f32>>,
    pub(crate) temperature_2m_min: Vec<Option<f32>>,
}
