-- Your SQL goes here

CREATE TABLE historical_weather_daily_records (
    "date" char(10) PRIMARY KEY,
    temperature_2m_max real,
    temperature_2m_min real,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW()
);
