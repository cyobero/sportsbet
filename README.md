# Sporstbet
A sports betting app written entirely in Rust

## Quick Usage
This requires having [Docker](https://docs.docker.com/get-docker/) installed

First, set up the database
```
docker-compose up db
```

Then,
```
docker-compose up --build -d
```

Open a browser and go to `localhost:8305`.
