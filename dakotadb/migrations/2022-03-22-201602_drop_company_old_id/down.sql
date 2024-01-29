-- This file should undo anything in `up.sql`
ALTER TABLE companies
ADD COLUMN old_id INTEGER;
