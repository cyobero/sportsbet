FROM rust:1.62

RUN cargo install diesel_cli --no-default-features --features postgres

ENV DATABASE_URL=postgresql://postgres:password123@172.17.0.1:5432/sportsbet_db

WORKDIR /usr/src/sporstsbet

COPY . . 

RUN diesel setup

RUN cargo install --path .

CMD ["sportsbet"]
