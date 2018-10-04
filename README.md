# Hermes
An ultra minimalist self-hosted RSS reader

## Requirements

- Postgres database

## Installation

The easiest way to get up and running is to use the bundled `docker-compose.yml` file.

```bash
docker-compose up -d
```

Then simply visit `http://localhost:3030` in your browser and log in. The `admin` user is created automatically, with the password specified in the docker-compose file, also `admin` by default.

Once the Hermes container has started for the first time, it's recommended to remove the `ADMIN_PASS=admin` line from the `docker-compose.yml` file as it is no longer needed.
