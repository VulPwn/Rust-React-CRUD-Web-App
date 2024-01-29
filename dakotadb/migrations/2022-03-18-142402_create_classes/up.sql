CREATE TABLE classes (
	id SERIAL PRIMARY KEY,
	class_title TEXT NOT NULL,
	class_language TEXT NOT NULL,
	md_approval_num TEXT,
	va_approval_num TEXT,
	dc_approval_num TEXT,
	md_recert_yrs INTEGER,
	va_recert_yrs INTEGER,
	dc_recert_yrs INTEGER
)
