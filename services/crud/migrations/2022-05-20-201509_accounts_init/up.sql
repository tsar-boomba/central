-- Your SQL goes here

CREATE TABLE public.accounts (
	id				TEXT		NOT NULL PRIMARY KEY,
	created_at		TIMESTAMP	NOT NULL DEFAULT NOW(),
	updated_at		TIMESTAMP	NOT NULL DEFAULT NOW(),
	address			TEXT		NOT	NULL,
	email			TEXT		NOT	NULL,
	business_name	TEXT		NOT NULL,
	short_name		TEXT		NOT	NULL,
    city			TEXT		NOT NULL,
    zip_code		TEXT		NOT NULL,
    phone_number	TEXT		NOT NULL
);

CREATE UNIQUE INDEX email_idx ON public.accounts (email);

SELECT diesel_manage_updated_at ('accounts');
