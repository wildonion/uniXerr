-- Your SQL goes here


CREATE TABLE users(
    id SERIAL NOT NULL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    wallet_address VARCHAR NOT NULL DEFAULT '', 
    access_token VARCHAR NOT NULL DEFAULT '',
    access_level SMALLINT DEFAULT 1 NOT NULL,
    is_blocked SMALLINT DEFAULT 0 NOT NULL, 
    phone_number VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    device_id VARCHAR NOT NULL,
    firebase_id VARCHAR DEFAULT '',
    prof_img VARCHAR DEFAULT '',
    coins INTEGER DEFAULT 100 NOT NULL,
    sex CHAR(1) DEFAULT 'n' NOT NULL,
    age SMALLINT DEFAULT 0 NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
)