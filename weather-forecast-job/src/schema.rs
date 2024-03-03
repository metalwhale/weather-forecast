// @generated automatically by Diesel CLI.

diesel::table! {
    historical_weather_daily_records (date) {
        #[max_length = 10]
        date -> Bpchar,
        temperature_2m_max -> Nullable<Float4>,
        temperature_2m_min -> Nullable<Float4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
