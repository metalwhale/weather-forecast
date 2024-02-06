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
Get inside the main container:
```bash
docker compose exec main bash
```

### Run the job
```bash
cargo run -p weather-forecast-job
```
