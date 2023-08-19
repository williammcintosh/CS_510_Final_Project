-- Add migration script here
UPDATE users
SET is_admin = TRUE
WHERE id = 1;