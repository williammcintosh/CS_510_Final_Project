DELETE FROM users;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('users', 'id'), 1, false);

INSERT INTO users(email, password) VALUES ('first@apods.com', '1qazxsw2');
INSERT INTO users(email, password) VALUES ('second@apods.com', '1qazxsw2');
INSERT INTO users(email, password) VALUES ('third@apods.com', '1qazxsw2');
INSERT INTO users(email, password) VALUES ('fourth@apods.com', '1qazxsw2');
