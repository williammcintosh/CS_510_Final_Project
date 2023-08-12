-- Add up migration script here
INSERT INTO questions(title, content, tags) VALUES ('Question One Title', 'Question One Content', ARRAY['tag1.1', 'tag1.2']);
INSERT INTO questions(title, content, tags) VALUES ('Question Two Title', 'Question Two Content', ARRAY['tag2.1', 'tag2.2']);
