[![CI status](https://drone-1.prima.it/api/badges/primait/veil/status.svg?branch=master)](https://drone-1.prima.it/primait/veil) *[Put your preferred code coverage badge here]*

# veil

Rust derive macro for masking sensitive data in `std::fmt::Debug`

## Run locally

The recommended way to work locally with veil is to open a shell inside the container and run all commands from there:

```bash
docker-compose run --service-ports web bash
```

### Set up environment variables

Copy the `./config/.env.dist.local` file to `./config/.env`. You change the latter if you need to do so.
The following environment variables are supported:

- **APP_ENV**: Environment where the application is deployed
- **DATADOG_FROM_ADDRESS**: Address of the udp socket it will bind to for sending
- **DATADOG_TO_ADDRESS**: Address of the udp socket it will send metrics and events to
- **OPENTELEMETRY_URL**: Address of the OpenTelemetry collector endpoint
- **DB_HOST**: Hostname of the Postgres DB
- **DB_PORT**: Port for the Postgres DB
- **DB_NAME**: Name of the DB
- **DB_USER**: User on the Postgres DB
- **DB_PASSWORD**: Password for user on the Postgres DB

### Set up the database

To create a new migration, inside the container run:

```bash
sqlx migrate add <migration_name>
```

Then to create the database and run migrations run:

```bash
cargo make db-setup
```

Alternatively, to drop the DB, recreate it and re-run migrations run:

```bash
cargo make db-reset
```

### Run

Inside the container, run the application as follows:

```bash
cargo make run
```

Tests can be run with:

```bash
cargo make test
```

### View traces locally

You can access the Jaeger Web UI at http://localhost:16686 to inspect the OpenTelemetry traces exported by veil.

You can also see the raw UDP messages sent by prima_datadog with the command `docker-compose logs datadog -f`.

### Testing with localauth0

You can test Auth0 integrations locally using `localauth0`. You can use it as an Auth0 authority, obtain JWTs and so on. You can find some examples of how to interact with it [here](https://github.com/primait/localauth0/tree/master/examples).
