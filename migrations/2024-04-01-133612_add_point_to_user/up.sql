ALTER TABLE users
ADD COLUMN coin_amount NUMERIC DEFAULT 0;

UPDATE users
SET coin_amount = 0;

ALTER TABLE users
ALTER COLUMN coin_amount SET NOT NULL;