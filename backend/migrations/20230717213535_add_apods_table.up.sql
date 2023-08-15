CREATE TABLE IF NOT EXISTS apods
(
    id         serial           PRIMARY KEY,
    title      VARCHAR(255)     NOT NULL,
    img_date   TEXT             UNIQUE NOT NULL,
    content    TEXT             NOT NULL,
    url        TEXT             NOT NULL,
    created_on TIMESTAMPTZ      NOT NULL DEFAULT NOW()
);
