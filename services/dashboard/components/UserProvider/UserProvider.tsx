import { api } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';
import { User } from '@/types/User';
import { useRouter } from 'next/router';
import {
	createContext,
	Dispatch,
	PropsWithChildren,
	SetStateAction,
	useContext,
	useState,
} from 'react';
import useSWR, { KeyedMutator } from 'swr';

interface UserContextValue {
	user: User | undefined;
	setFallback: Dispatch<SetStateAction<User | undefined>>;
	mutate: KeyedMutator<User>;
	error: any;
	isLoading: boolean;
	isValidating: boolean;
}

const UserContext = createContext<UserContextValue>({
	user: undefined,
	setFallback: () => {},
	mutate: () => Promise.resolve(undefined),
	error: undefined,
	isLoading: false,
	isValidating: false,
});

const UserProvider: React.FC<PropsWithChildren<unknown>> = ({ children }) => {
	const router = useRouter();
	const [fallback, setFallback] = useState<User | undefined>(undefined);
	const { data, error, isLoading, isValidating, mutate } = useSWR<User>(
		api('verify'),
		fetcher,
		fallback
			? {
					fallbackData: fallback,
					keepPreviousData: true,
			  }
			: { keepPreviousData: true },
	);

	if ((!data && !isLoading) || error) {
		if (router.pathname !== '/login') router.push(`/login?from=${location.pathname}`);
	}

	return (
		<UserContext.Provider
			value={{ user: data, mutate, error, isLoading, isValidating, setFallback }}
		>
			{children}
		</UserContext.Provider>
	);
};

export const useUser = () => useContext(UserContext);

export const useSetFallbackUser = (user: User | undefined) => useUser().setFallback(user);

export default UserProvider;
