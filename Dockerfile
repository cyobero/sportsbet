FROM rust:1.63

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/sporstsbet

COPY . . 

RUN diesel setup

RUN cargo install --path .

ENV DATABASE_URL postgresql://postgres:password123@db/sportsbet_db

CMD ["sportsbet"]
