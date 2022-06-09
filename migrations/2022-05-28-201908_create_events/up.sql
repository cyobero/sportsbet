-- Your SQL goes here
CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    home VARCHAR(3) NOT NULL,
    away VARCHAR(3) NOT NULL,
    start TIMESTAMP NOT NULL
);


CREATE TABLE game_results (
    id SERIAL PRIMARY KEY,
    home INT NOT NULL,
    away INT NOT NULL,
    game_id INT NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games (id) ON DELETE CASCADE
);


CREATE TABLE events (
    id SERIAL NOT NULL,
    description VARCHAR(127) NOT NULL,
    odds INT NOT NULL,
    result_id INT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id, timestamp),
    FOREIGN KEY (result_id) REFERENCES game_results(id) ON DELETE CASCADE
);
