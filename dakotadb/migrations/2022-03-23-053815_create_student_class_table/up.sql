-- Your SQL goes here
CREATE TABLE studentclass (
	id SERIAL PRIMARY KEY,
	class_id INTEGER NOT NULL,
	student_id INTEGER NOT NULL,
	certification_num TEXT,
	test_score INTEGER,
	class_date DATE NOT NULL DEFAULT CURRENT_DATE,
	class_end_date DATE NOT NULL DEFAULT CURRENT_DATE,
	md_recert_date DATE NOT NULL DEFAULT CURRENT_DATE,
	va_recert_date DATE NOT NULL DEFAULT CURRENT_DATE,
	dc_recert_date DATE NOT NULL DEFAULT CURRENT_DATE
)

