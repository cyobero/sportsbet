-- Your SQL goes here
CREATE TYPE role AS ENUM ('bookie', 'punter');
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(127) NOT NULL,
    username VARCHAR(127) NOT NULL,
    password VARCHAR(255) NOT NULL,
    role ROLE NOT NULL DEFAULT 'punter',
    UNIQUE(email, username)
);
