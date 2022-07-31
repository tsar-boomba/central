import useSWR from 'swr';
import { useUser } from '../components/UserProvider';
import { api } from './apiHelpers';
import fetcher from './swrFetcher';

/** Checks current account's subscription, assumes they are subbed (first render will always be true) */
export const useIsSubbed = () => {
	const { user } = useUser();
	const { data } = useSWR<boolean>(
		user ? api(`accounts/${user.accountId}/is-subbed`) : null,
		fetcher,
		{
			fallbackData: true,
			revalidateOnFocus: false,
			revalidateOnReconnect: false,
			revalidateIfStale: false,
			revalidateOnMount: true,
		},
	);
	return data;
};
