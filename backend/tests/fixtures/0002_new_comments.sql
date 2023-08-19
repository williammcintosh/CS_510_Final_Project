-- Add migration script here
DELETE FROM comments;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('comments', 'id'), 1, false);

INSERT INTO comments(content, apod_id, user_id) VALUES ('Such a cool picture!', 1, 1);
INSERT INTO comments(content, apod_id, user_id) VALUES ('Been there. Overrated.', 2, 1);
INSERT INTO comments(content, apod_id, user_id) VALUES ('This is amazing!', 3, 3);
INSERT INTO comments(content, apod_id, user_id) VALUES ('Absolutely wonderful!', 4, 4);
