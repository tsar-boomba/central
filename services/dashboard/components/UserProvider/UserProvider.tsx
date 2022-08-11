import { api } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';
import { User } from '@/types/User';
import { useRouter } from 'next/router';
import { createContext, ReactNode, useContext } from 'react';
import useSWR, { KeyedMutator } from 'swr';
import { Account } from '@/types/Account';

interface UserContextValue {
	user: User | undefined;
	mutate: KeyedMutator<User>;
	error: any;
	isLoading: boolean;
	isValidating: boolean;
}

const UserContext = createContext<UserContextValue>({
	user: undefined,
	mutate: () => Promise.resolve(undefined),
	error: undefined,
	isLoading: false,
	isValidating: false,
});

const publicPaths = ['/login', '/register'];

interface Props {
	children: ReactNode;
	fallback?: User;
}

const UserProvider = ({ children, fallback }: Props) => {
	const router = useRouter();
	const isPublic = publicPaths.includes(router.pathname);
	const { data, error, isLoading, isValidating, mutate } = useSWR<User>(
		!isPublic ? api('verify') : null,
		fetcher,
		fallback
			? {
					fallbackData: fallback,
					keepPreviousData: true,
			  }
			: { keepPreviousData: true },
	);

	if ((!data && !isLoading) || error) {
		if (!isPublic) router.push(`/login?from=${location.pathname}`);
	}

	return (
		<UserContext.Provider value={{ user: data, mutate, error, isLoading, isValidating }}>
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
