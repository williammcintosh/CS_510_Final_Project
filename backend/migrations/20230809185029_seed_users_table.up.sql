-- Add up migration script here
INSERT INTO users (email, password, is_admin)
VALUES
    ('willymac@legitemail.com', '1234qwer', true),
    ('billyjoel@legitemail.com', '1qazxsw2', false);