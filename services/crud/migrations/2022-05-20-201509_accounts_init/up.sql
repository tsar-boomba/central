-- Your SQL goes here
CREATE TYPE Resource AS ENUM ('load', 'carrier', 'shipper');

CREATE TYPE Role AS ENUM('owner', 'admin', 'moderator', 'user');

CREATE TABLE public.users (
	id				SERIAL		NOT NULL PRIMARY KEY,
	created_at		TIMESTAMP	NOT NULL DEFAULT NOW(),
	updated_at		TIMESTAMP	NOT NULL DEFAULT NOW(),
	account_id		TEXT		NOT NULL,
	username		TEXT		NOT NULL,
	first_name		TEXT		NOT	NULL,
	last_name		TEXT		NOT NULL,
	password		TEXT		NOT NULL,
	active			BOOLEAN		NOT NULL DEFAULT 't',
	instances		TEXT[]		NOT NULL DEFAULT '{}',
	create_perms	Resource[]	NOT NULL DEFAULT '{}',
	update_perms	Resource[]	NOT NULL DEFAULT '{}',
	delete_perms	Resource[]	NOT NULL DEFAULT '{}',
	role			Role		NOT	NULL DEFAULT 'user'::Role,
	notes			TEXT
);

SELECT diesel_manage_updated_at ('users');

CREATE UNIQUE INDEX username_account_id_idx ON public.users (username, account_id);

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
    phone_number	TEXT		NOT NULL,
	stripe_id		TEXT		NOT NULL UNIQUE,
	state			TEXT		NOT NULL
);

CREATE UNIQUE INDEX email_idx ON public.accounts (email);

SELECT diesel_manage_updated_at ('accounts');
