-- Your SQL goes here
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    description VARCHAR(255) NOT NULL,
    odds INT NOT NULL
);
