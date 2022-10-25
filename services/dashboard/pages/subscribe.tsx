import { loadStripe } from '@stripe/stripe-js';
import { Elements } from '@stripe/react-stripe-js';
import { Container, Loader, Text } from '@mantine/core';
import { useUser } from '@/utils/authUtils';
import { api, ssrFetch } from '../utils/apiHelpers';
import useSWR from 'swr';
import fetcher from '../utils/swrFetcher';
import { GetServerSideProps } from 'next';
import { isAuthed, redirect, requireRole } from '../utils/authUtils';
import { Account } from '../types/Account';
import SubscribeForm from '../components/Form/SubscribeForm';
import { useSubStatus } from '@/utils/useSubStatus';
import { Role } from '@/types/User';

// Make sure to call loadStripe outside of a component’s render to avoid
// recreating the Stripe object on every render.
// This is your test publishable API key.
const stripePromise = loadStripe(process.env.NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY || '');

interface Props {
	error?: string;
	account?: Account;
}

const Subscribe = ({ error, account: fallback }: Props) => {
	const { user } = useUser();
	const { data: account } = useSWR<Account>(
		user?.accountId ? api(`accounts/${user.accountId}`) : null,
		fetcher,
		{ fallbackData: fallback },
	);
	const { status } = useSubStatus();

	if (status !== undefined)
		return (
			<Text sx={{ fontSize: 42 }} component='h1'>
				You are already subscribed.
			</Text>
		);
	if (!user || !account) return <Loader />;
	if (error || !account?.stripeId) return <Text>An error ocurred, please try again.</Text>;

	return (
		<Container>
			<Elements stripe={stripePromise}>
				<SubscribeForm />
			</Elements>
		</Container>
	);
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
	const user = await isAuthed(ctx);
	console.log(user);

	if (!user) {
		return redirect('/subscribe');
	}

	if (!requireRole(user.role, Role.Owner)) {
		return redirect('/');
	}

	// When they visit this page, get their account and see if it has customer & subscription
	const accountRes = await ssrFetch(api(`accounts/${user.accountId}`), ctx);
	console.log('acctRes:', accountRes);

	if (!accountRes.ok)
		return {
			props: {
				error: (await accountRes.json())?.message || 'An error ocurred creating customer.',
			},
		};

	const account: Account = await accountRes.json();
	console.log(account);
	if (!account.stripeId) {
		const customerRes = await ssrFetch(api('payments/customer'), ctx, {
			body: JSON.stringify(account),
			headers: {
				'Content-Type': 'application/json',
			},
			method: 'POST',
		});
		console.log('custRes:', customerRes);

		if (!customerRes.ok)
			return {
				props: {
					error:
						(await customerRes.json())?.message ||
						'An error ocurred creating customer.',
				},
			};

		const customerId = await customerRes.text();
		account.stripeId = customerId;
	}

	return { props: { account } };
};

export default Subscribe;
