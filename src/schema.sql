CREATE TYPE job_status AS ENUM ('not_appled', 'appled', 'rejected', 'accepted');
CREATE TABLE users (
	user_id SERIAL PRIMARY KEY,
	username VARCHAR(50) UNIQUE NOT NULL,
	email VARCHAR(100) UNIQUE NOT NULL,
	password_hash varchar(255)
);


CREATE TABLE jobs (
	job_id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
	date_applyed TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	contact_info JSONB,
	notes TEXT,
	status job_status DEFAULT 'not_appled'
);

