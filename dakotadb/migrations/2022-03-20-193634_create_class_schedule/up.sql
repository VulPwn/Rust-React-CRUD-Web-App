-- Your SQL goes here
CREATE TABLE class_schedule (
	id SERIAL PRIMARY KEY,
	class_id INTEGER NOT NULL,
	class_date DATE NOT NULL DEFAULT CURRENT_DATE,
	class_end_date DATE NOT NULL DEFAULT CURRENT_DATE,
	md_recert_date DATE NOT NULL DEFAULT CURRENT_DATE,
	va_recert_date DATE NOT NULL DEFAULT CURRENT_DATE,
	dc_recert_date DATE NOT NULL DEFAULT CURRENT_DATE,
	instructor_id INTEGER NOT NULL,
	second_instructor_id INTEGER,
	cancelled BOOLEAN NOT NULL DEFAULT FALSE
)
