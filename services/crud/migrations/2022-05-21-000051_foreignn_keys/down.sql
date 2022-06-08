-- This file should undo anything in `up.sql`

ALTER TABLE public.users DROP CONSTRAINT fk_account_user;

ALTER TABLE public.instances DROP CONSTRAINT fk_account_instance;
