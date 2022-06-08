-- Your SQL goes here

ALTER TABLE public.users
	ADD CONSTRAINT fk_account_user
	FOREIGN KEY(account_id)
	REFERENCES public.accounts (id)
	ON DELETE CASCADE;

ALTER TABLE public.instances
	ADD CONSTRAINT fk_account_instance
	FOREIGN KEY(account_id)
	REFERENCES public.accounts (id)
	ON DELETE CASCADE;
