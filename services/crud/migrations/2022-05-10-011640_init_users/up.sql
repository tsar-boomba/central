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
