CREATE TABLE IF NOT EXISTS comments
(
   id                     serial PRIMARY KEY,
    content                TEXT      NOT NULL,
    created_on             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    apod_id            integer REFERENCES apods ON DELETE CASCADE,
    user_id                integer REFERENCES users ON DELETE CASCADE
);