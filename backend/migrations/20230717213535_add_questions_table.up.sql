CREATE TABLE IF NOT EXISTS questions
(
    id         serial PRIMARY KEY,
    title      VARCHAR(255) NOT NULL,
    img_date   TEXT         NOT NULL,
    content    TEXT         NOT NULL,
    url        TEXT         NOT NULL,
    tags       TEXT[],
    created_on TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);
