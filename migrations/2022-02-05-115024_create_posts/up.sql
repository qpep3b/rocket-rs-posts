-- Your SQL goes here
BEGIN TRANSACTION;

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    login VARCHAR(32) NOT NULL,
    secret_password VARCHAR(128) NOT NULL
);

CREATE TABLE posts (
    post_id SERIAL PRIMARY KEY,
    title VARCHAR(64) NOT NULL,
    content TEXT NOT NULL,
    author_id INTEGER NOT NULL,
    FOREIGN KEY(author_id) REFERENCES users(user_id)
);

COMMIT;
