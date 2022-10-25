import { Account } from '@/types/Account';
import { Role, User } from '@/types/User';
import { GetServerSideProps, GetServerSidePropsResult } from 'next';
import useSWR from 'swr';
import { api } from './apiHelpers';
import fetcher from './swrFetcher';

export const isAuthed = async ({
	req,
}: {
	req: { cookies: { [key: string]: string | undefined } };
}) => {
	return new Promise<User | undefined>((resolve) =>
		fetch(api('verify'), {
			headers: { Cookie: `at=${req.cookies.at}` },
		})
			.then((res) => {
				if (res.ok) {
					res.json().then((json) => {
						resolve(json);
					});
				} else {
					resolve(undefined);
				}
			})
			.catch(() => resolve(undefined)),
	);
};

export const redirect = <P>(from = '', path = '/login'): GetServerSidePropsResult<P> => ({
	redirect: {
		destination: `${path}${from !== '' ? `?from=${from}` : ''}`,
		permanent: false,
	},
});

export const DEFAULT_SSR_RETURN: GetServerSidePropsResult<Record<string, unknown>> = { props: {} };

export const DEFAULT_SSR =
	(from?: string, requiredRole: Role = Role.User): GetServerSideProps =>
	async (ctx) => {
		const user = await isAuthed(ctx);
		if (user) {
			if (!requireRole(user.role, requiredRole)) {
				return redirect('', '/'); // cant just go back to current page 😔
			}
			return DEFAULT_SSR_RETURN;
		} else {
			return redirect(from);
		}
	};

const roleToNum = (role: Role) => {
	switch (role) {
		case Role.Owner:
			return 3;
		case Role.User:
			return 0;
		case Role.Admin:
			return 2;
		case Role.Moderator:
			return 1;
		default:
			return 0;
	}
};

export const requireRole = (curr: Role | undefined, required: Role): boolean => {
	if (curr) {
		return roleToNum(curr) >= roleToNum(required);
	}
	return false;
};

export const higherRole = (curr: Role | undefined, required: Role): boolean => {
	if (curr) {
		return roleToNum(curr) > roleToNum(required);
	}
	return false;
};

export const useUser = () => {
	const { data, ...rest } = useSWR<User>(api('verify'), fetcher);
	return { user: data, ...rest };
};

export const useAccount = () => {
	const { user } = useUser();
	const { data, ...rest } = useSWR<Account>(
		user ? api(`accounts/${user.accountId}`) : null,
		fetcher,
	);
	return { account: data, ...rest };
};
