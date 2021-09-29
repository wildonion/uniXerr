-- Your SQL goes here


CREATE TABLE login_history(
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    last_login TIMESTAMP NOT NULL
)