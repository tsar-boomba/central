import { api } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';
import { User } from '@/types/User';
import { useRouter } from 'next/router';
import { createContext, ReactNode, useCallback, useContext } from 'react';
import useSWR, { KeyedMutator, mutate as SWRMutate } from 'swr';
import { Account } from '@/types/Account';

interface UserContextValue {
	user: User | undefined;
	mutate: KeyedMutator<User>;
	error: any;
}

const UserContext = createContext<UserContextValue>({
	user: undefined,
	mutate: () => Promise.resolve(undefined),
	error: undefined,
});

const publicPaths = ['/login', '/register'];

interface Props {
	children: ReactNode;
	fallback?: User;
}

const UserProvider = ({ children, fallback }: Props) => {
	const router = useRouter();
	const isPublic = publicPaths.includes(router.pathname);
	const { data, error } = useSWR<User>(
		!isPublic ? api('verify') : null,
		fetcher,
		fallback
			? {
					fallbackData: fallback,
					keepPreviousData: true,
			  }
			: { keepPreviousData: true },
	);
	// avoid rerenders
	const mutate = useCallback<KeyedMutator<User>>(
		(data, opts) => SWRMutate<User>(api('verify'), data, opts),
		[],
	);

	if (!data || error) {
		if (!isPublic) router.push(`/login?from=${router.asPath || router.pathname}`);
	}

	return (
		<UserContext.Provider value={{ user: data, mutate, error }}>
			{children}
		</UserContext.Provider>
	);
};

export const useUser = () => useContext(UserContext);

export const useAccount = () => {
	const { user } = useUser();
	const { data: account, ...rest } = useSWR<Account>(
		user ? api(`accounts/${user.accountId}`) : null,
		fetcher,
	);
	return { account, ...rest };
};

export default UserProvider;
