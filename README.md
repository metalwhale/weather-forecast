# weather-forecast

## Local development
### Startup
Create a `.env` file by copying from [`./.env.local`](./.env.local) and then fill in the variables with appropriate values:
```bash
cp .env.local .env
```
Start the containers:
```bash
docker compose up -d --build --remove-orphans
```
Get inside the app container:
```bash
docker compose exec app bash
```

### Job
Run migrations:
```bash
cd ./weather-forecast-job/
DATABASE_URL=postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_DB} diesel migration run
cd ../
```
Run the job:
```bash
cargo run -p weather-forecast-job
```
