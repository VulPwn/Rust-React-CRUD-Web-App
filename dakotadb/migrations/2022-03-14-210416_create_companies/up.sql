-- Your SQL goes here
CREATE TABLE companies (
	id SERIAL PRIMARY KEY,
	company_name TEXT NOT NULL,
	address TEXT NOT NULL,
	suite TEXT,
	city TEXT NOT NULL,
	state TEXT NOT NULL,
	zip_code TEXT NOT NULL,
	phone TEXT NOT NULL,
	phone_ext TEXT,
	email TEXT NOT NULL,
	poc_firstName TEXT,
	poc_lastName TEXT,
	cc_holderName TEXT,
	cc_num TEXT,
	cc_expDate TEXT,
	cc_cvv TEXT,
	cc_zipCode TEXT,
	notes TEXT,
	old_id INTEGER
)
