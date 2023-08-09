DELETE FROM questions;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('questions', 'id'), 1, false);

INSERT INTO questions(title, content, tags) VALUES ('TestTitle1', 'Question Content', ARRAY['tag1', 'tag2']);
INSERT INTO questions(title, content, tags) VALUES ('TestTitle2', 'Another Question Content', ARRAY['tag1', 'tag2']);
INSERT INTO questions(title, content, tags) VALUES ('TestTitle3', 'Question Content', ARRAY['tag1', 'tag2']);
INSERT INTO questions(title, content) VALUES ('TestTitle4', 'Another Question Content');
