import useSWR from 'swr';
import { useAccount } from '../components/UserProvider';
import { api } from './apiHelpers';

const textFetcher = async <Text>(url: string, init: RequestInit = {}): Promise<Text> => {
	const res = await fetch(url, { ...init, credentials: 'include', mode: 'cors' });

	if (!res.ok) {
		const err = new Error(
			(await res.json()).message || 'An error occurred while fetching data.',
		);
		throw err;
	}

	return res.text() as Text;
};

/** Checks current account's subscription, assumes they are active, if undefined, they dont have a sub  */
export const useSubStatus = () => {
	const { account } = useAccount();
	const { data: status, ...rest } = useSWR<'active' | 'unpaid' | 'past_due'>(
		account ? api(`payments/sub-status?subId=${account.subId}`) : null,
		textFetcher,
		{
			fallbackData: 'active',
			revalidateOnFocus: false,
			revalidateOnReconnect: false,
			revalidateIfStale: false,
			revalidateOnMount: true,
		},
	);

	return { status: rest.error ? undefined : status, ...rest };
};
