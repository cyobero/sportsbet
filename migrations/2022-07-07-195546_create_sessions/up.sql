-- Your SQL goes here
CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    login_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    logout_date TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
