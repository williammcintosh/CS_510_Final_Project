DELETE FROM apods;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('apods', 'id'), 1, false);

INSERT INTO apods(title, content) VALUES ('TestTitle1', 'Apod Content');
INSERT INTO apods(title, content) VALUES ('TestTitle2', 'Another Apod Content');
INSERT INTO apods(title, content) VALUES ('TestTitle3', 'Apod Content');
INSERT INTO apods(title, content) VALUES ('TestTitle4', 'Another Apod Content');
