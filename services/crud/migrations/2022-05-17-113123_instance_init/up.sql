-- Your SQL goes here

CREATE TABLE public.instances (
	id				TEXT		NOT NULL PRIMARY KEY,
	created_at		TIMESTAMP	NOT NULL DEFAULT NOW(),
	updated_at		TIMESTAMP	NOT NULL DEFAULT NOW(),
	account_id		TEXT		NOT NULL,
	db_url			TEXT		NOT NULL,
	url				TEXT		NOT	NULL,
	business_name	TEXT		NOT NULL,
	short_name		TEXT		NOT	NULL,
    address			TEXT		NOT NULL,
    city			TEXT		NOT NULL,
    zip_code		TEXT		NOT NULL,
    phone_number	TEXT		NOT NULL,
	rate_conf_email	TEXT		NOT	NULL,
	instance_name	TEXT,
	top_terms		TEXT,
	bottom_terms	TEXT[]
);

SELECT diesel_manage_updated_at ('instances');

CREATE INDEX instances_account_id ON public.instances (account_id);
