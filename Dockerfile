FROM rust:1.62

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/sporstsbet

COPY . . 

RUN diesel setup

RUN cargo install --path .

CMD ["sportsbet"]
