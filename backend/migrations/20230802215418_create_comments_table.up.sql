CREATE TABLE IF NOT EXISTS comments
(
   id                     serial PRIMARY KEY,
    content                TEXT      NOT NULL,
    created_on             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    answer_id              integer REFERENCES answers ON DELETE CASCADE,
    question_id            integer REFERENCES questions ON DELETE CASCADE,

    CONSTRAINT only_one_foreign_key CHECK (
        (question_id IS NOT NULL AND answer_id IS NULL)
        OR
        (question_id IS NULL AND answer_id IS NOT NULL)
    )
);

COMMENT ON CONSTRAINT only_one_foreign_key ON comments IS 'Ensure each comment is associated with either a question or an answer, but not both';
