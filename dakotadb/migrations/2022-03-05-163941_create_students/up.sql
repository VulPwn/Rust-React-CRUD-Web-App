-- Your SQL goes here
CREATE TABLE students (
	id SERIAL PRIMARY KEY,
	social TEXT NOT NULL,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	address TEXT NOT NULL,
	suite TEXT,
	city TEXT NOT NULL,
	state TEXT NOT NULL,
	zip_code TEXT NOT NULL,
	phone TEXT NOT NULL,
	dob DATE NOT NULL DEFAULT CURRENT_DATE,
	company_id INTEGER,
	email TEXT,
	photo TEXT,
	old_id INTEGER
)
