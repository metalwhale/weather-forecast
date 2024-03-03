use diesel::PgConnection;

pub(crate) mod historical_weather;

pub(crate) struct Repository {
    connection: PgConnection,
}
