-- Your SQL goes here
CREATE TYPE Resource AS ENUM ('load', 'carrier', 'shipper');

CREATE TYPE Role AS ENUM('owner', 'admin', 'moderator', 'user');

CREATE TABLE public.users (
	id				TEXT		NOT NULL PRIMARY KEY,
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

CREATE TYPE InstanceStatus AS ENUM ('deploying', 'failed', 'ok', 'unhealthy', 'inactive', 'configured');

CREATE TABLE public.instances (
	id				TEXT			NOT NULL PRIMARY KEY,
	created_at		TIMESTAMP		NOT NULL DEFAULT NOW(),
	updated_at		TIMESTAMP		NOT NULL DEFAULT NOW(),
	account_id		TEXT			NOT NULL,
	url				TEXT,
	name			TEXT			NOT NULL,
	status			InstanceStatus	NOT NULL,
	business_name	TEXT			NOT NULL,
	short_name		TEXT			NOT	NULL,
    address1		TEXT			NOT NULL,
	address2		TEXT,
    city			TEXT			NOT NULL,
    zip_code		TEXT			NOT NULL,
	state			TEXT			NOT NULL,
    phone_number	TEXT			NOT NULL,
	email			TEXT			NOT	NULL,
	env_id			TEXT,
	key				TEXT,
	instance_name	TEXT,
	top_text		TEXT,
	bottom_text		TEXT
);

SELECT diesel_manage_updated_at ('instances');

CREATE INDEX instances_account_id ON public.instances (account_id);

CREATE UNIQUE INDEX name_account_id_idx ON public.instances (name, account_id);

CREATE TABLE public.accounts (
	id				TEXT		NOT NULL PRIMARY KEY,
	created_at		TIMESTAMP	NOT NULL DEFAULT NOW(),
	updated_at		TIMESTAMP	NOT NULL DEFAULT NOW(),
	address1		TEXT		NOT NULL,
	address2		TEXT,
	email			TEXT		NOT	NULL,
	business_name	TEXT		NOT NULL,
	short_name		TEXT		NOT	NULL,
    city			TEXT		NOT NULL,
    zip_code		TEXT		NOT NULL,
    phone_number	TEXT		NOT NULL,
	state			TEXT		NOT NULL,
	stripe_id		TEXT,
	sub_id			TEXT
);

CREATE UNIQUE INDEX email_idx ON public.accounts (email);

SELECT diesel_manage_updated_at ('accounts');
