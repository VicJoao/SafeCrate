-- Your SQL goes here
CREATE TABLE file (
    id SERIAL PRIMARY KEY,
    -- owner is a user from table users
    owner_id INTEGER NOT NULL,
    -- name of the file
    name VARCHAR(255) NOT NULL,
    -- path to the file
    path VARCHAR(255) NOT NULL,
    --AES key for the file
    key VARCHAR(255) NOT NULL,
    -- created_at is the time the file was created
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (owner_id) REFERENCES users(id)
);