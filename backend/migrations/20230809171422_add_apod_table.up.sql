-- Add up migration script here
CREATE TABLE IF NOT EXISTS apods
(
    id              serial          PRIMARY KEY,
    img_date            TIMESTAMPTZ     NOT NULL,
    explanation     TEXT            NOT NULL,
    title           VARCHAR(255)    NOT NULL,
    url             TEXT            NOT NULL
);