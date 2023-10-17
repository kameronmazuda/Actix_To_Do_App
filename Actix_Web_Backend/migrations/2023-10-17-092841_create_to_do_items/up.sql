CREATE TABLE to_do (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	status varchar NOT NULL,
	date timestamp NOT NULL DEFAULT NOW()
)
