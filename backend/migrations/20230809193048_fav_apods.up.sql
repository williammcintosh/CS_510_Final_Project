-- Add up migration script here
CREATE TABLE IF NOT EXISTS fav_apods
(
    id          serial PRIMARY KEY,
    user_id     integer REFERENCES users(id),
    apod_id     integer REFERENCES apods(id)
    );