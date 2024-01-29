-- Your SQL goes here
CREATE TABLE instructors (
	id SERIAL PRIMARY KEY,
	acronym TEXT NOT NULL,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL
)
