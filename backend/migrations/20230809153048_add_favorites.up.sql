-- Add up migration script here
CREATE TABLE IF NOT EXISTS favorites
(
    id                 serial       PRIMARY KEY,
    apod_id            integer      REFERENCES apods ON DELETE CASCADE,
    user_id            integer      REFERENCES users ON DELETE CASCADE,

    CONSTRAINT unique_favorite_entry UNIQUE (apod_id, user_id)
);

COMMENT ON CONSTRAINT unique_favorite_entry ON favorites IS 'Ensure that there are no duplicate (apod_id, user_id) pairs!';