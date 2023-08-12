-- Add up migration script here
INSERT INTO comments(content, question_id, answer_id) VALUES ('First comment about question 1', 1, NULL);
INSERT INTO comments(content, question_id, answer_id) VALUES ('Second comment about question 1', 1, NULL);
INSERT INTO comments(content, question_id, answer_id) VALUES ('Third comment about question 1', 1, NULL);
INSERT INTO comments(content, question_id, answer_id) VALUES ('First comment about answer 1', NULL, 1);
INSERT INTO comments(content, question_id, answer_id) VALUES ('Second comment about answer 1', NULL, 1);
INSERT INTO comments(content, question_id, answer_id) VALUES ('Third comment about answer 1', NULL, 1);