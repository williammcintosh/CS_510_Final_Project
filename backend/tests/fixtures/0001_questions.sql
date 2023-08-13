DELETE FROM questions;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('questions', 'id'), 1, false);

INSERT INTO questions(title, content) VALUES ('TestTitle1', 'Question Content');
INSERT INTO questions(title, content) VALUES ('TestTitle2', 'Another Question Content');
INSERT INTO questions(title, content) VALUES ('TestTitle3', 'Question Content');
INSERT INTO questions(title, content) VALUES ('TestTitle4', 'Another Question Content');
