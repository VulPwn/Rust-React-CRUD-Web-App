-- This file should undo anything in `up.sql`
ALTER TABLE students
ADD COLUMN old_id INTEGER;
