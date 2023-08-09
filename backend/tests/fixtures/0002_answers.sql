-- Delete everything currently in there so we start fresh
DELETE FROM answers;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('answers', 'id'), 1, false);

INSERT INTO answers(content, question_id) VALUES ('some content 1', 1);
INSERT INTO answers(content, question_id) VALUES ('Other Content', 1);
INSERT INTO answers(content, question_id) VALUES ('some content 2', 2);
INSERT INTO answers(content, question_id) VALUES ('some content 3', 3);
