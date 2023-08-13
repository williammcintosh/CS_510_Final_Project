CREATE TABLE IF NOT EXISTS comments
(
   id                     serial PRIMARY KEY,
    content                TEXT      NOT NULL,
    created_on             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    question_id            integer REFERENCES questions ON DELETE CASCADE
);