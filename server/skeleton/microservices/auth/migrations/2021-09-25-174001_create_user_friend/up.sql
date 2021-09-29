-- Your SQL goes here



CREATE TABLE user_friend(
    id SERIAL NOT NULL PRIMARY KEY,
    from_user_id INTEGER NOT NULL,
    to_friend_id INTEGER NOT NULL,
    status SMALLINT NOT NULL DEFAULT '0', -- 0 means request from_user_id to_friend_id is not accepted
    requested TIMESTAMP NOT NULL
)