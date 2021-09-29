-- Your SQL goes here


CREATE TABLE user_match(
    id SERIAL NOT NULL,
    user_id INTEGER NOT NULL,
    entrance_coins INTEGER NOT NULL,
    reward_coins INTEGER NOT NULL,
    match_id INTEGER NOT NULL,
    match_type VARCHAR NOT NULL,
    status SMALLINT NOT NULL DEFAULT '0',
    PRIMARY KEY (id, match_id, match_type)
)