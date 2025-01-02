-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    nickname VARCHAR(255) NOT NULL,
    public_key VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
