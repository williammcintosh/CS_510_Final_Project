-- Add up migration script here
CREATE TABLE IF NOT EXISTS favorites
(
    id                     serial PRIMARY KEY,
    question_id            integer REFERENCES questions ON DELETE CASCADE,
    user_id                integer REFERENCES users ON DELETE CASCADE
);