-- Your SQL goes here
CREATE TABLE file_share (
    id SERIAL PRIMARY KEY,
    -- file_id is a file from table file
    file_id INTEGER NOT NULL,
    -- user_id is a user from table users
    user_id INTEGER NOT NULL,
    -- AES_ENCRYPTED key for the file recrypted with the public key of the user
    key VARCHAR(255) NOT NULL,
    -- created_at is the time the file was shared
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (file_id) REFERENCES file(id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    UNIQUE (file_id, user_id)
);