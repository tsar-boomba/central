import useSWR from 'swr';
import { useAccount } from '@/utils/authUtils';
import { api } from './apiHelpers';

const textFetcher = async <Text>(url: string, init: RequestInit = {}): Promise<Text> => {
	const res = await fetch(url, { ...init, credentials: 'include', mode: 'cors' });

	if (!res.ok) {
		const err = new Error(
			(await res.json()).message || 'An error occurred while fetching data.',
		);
		throw err;
	}

	return res.text() as any as Text;
};

export type SubscriptionStatus = 'active' | 'unpaid' | 'past_due';

/** Checks current account's subscription, assumes they are active, if undefined, they dont have a sub  */
export const useSubStatus = () => {
	const { account } = useAccount();
	const { data: status, ...rest } = useSWR<SubscriptionStatus>(
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
	const realStatus = rest.error ? undefined : status;

	return {
		status: realStatus,
		isSubbed: realStatus !== undefined && realStatus !== 'unpaid',
		...rest,
	};
};

export const statusToText = (status: SubscriptionStatus) => {
	switch (status) {
		case 'active':
			return 'Active';
		case 'past_due':
			return 'Past Due';
		case 'unpaid':
			return 'Unpaid';
		default:
			return 'Unknown Status';
	}
};
