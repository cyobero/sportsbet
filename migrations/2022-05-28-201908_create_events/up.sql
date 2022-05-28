-- Your SQL goes here
CREATE TABLE events (
    id SERIAL NOT NULL,
    description VARCHAR(255) NOT NULL,
    odds INT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id, timestamp)
);
