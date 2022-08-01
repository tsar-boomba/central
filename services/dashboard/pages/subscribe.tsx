import { loadStripe, StripeElementsOptions } from '@stripe/stripe-js';
import { Elements } from '@stripe/react-stripe-js';
import { useEffect, useState } from 'react';
import { Loader, Text, useMantineTheme } from '@mantine/core';
import { useUser } from '../components/UserProvider';
import { api, callApi, ssrFetch } from '../utils/apiHelpers';
import useSWR from 'swr';
import fetcher from '../utils/swrFetcher';
import { GetServerSideProps } from 'next';
import { isAuthed, redirect } from '../utils/authUtils';
import { Account } from '../types/Account';
import SubscribeForm from '../components/Form/SubscribeForm';

// Make sure to call loadStripe outside of a component’s render to avoid
// recreating the Stripe object on every render.
// This is your test publishable API key.
const stripePromise = loadStripe(process.env.NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY || '');

interface Props {
	error?: string;
	account?: Account;
}

let calledSubscribe = false;

const Subscribe = ({ error, account: fallback }: Props) => {
	const [clientSecret, setClientSecret] = useState('');
	const { user } = useUser();
	const { data: account, mutate } = useSWR<Account>(
		user?.accountId ? api(`accounts/${user.accountId}`) : null,
		fetcher,
		{ fallbackData: fallback },
	);
	const theme = useMantineTheme();

	useEffect(() => {
		if (!account) return;
		// Create PaymentIntent as soon as the page loads
		!calledSubscribe &&
			callApi({ route: 'payments/subscribe', body: account })
				.then((res) => res.json())
				.then((data) => {
					mutate();
					setClientSecret(data.clientSecret);
				});
		calledSubscribe = true;
	}, []);

	if (!user || !account) return <Loader />;
	if (error || !account?.stripeId) return <Text>An error ocurred, please try again.</Text>;

	const appearance: StripeElementsOptions['appearance'] = {
		theme: 'none',
		variables: {
			colorPrimary: theme.colors[theme.primaryColor][theme.fn.primaryShade()],
			colorBackground: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.white,
			colorText: theme.colorScheme === 'dark' ? 'white' : 'black',
			borderRadius:
				typeof theme.fn.radius('sm') === 'string'
					? (theme.fn.radius('sm') as string)
					: `${theme.fn.radius('sm')}px`,
			focusOutline: theme.colors[theme.primaryColor][theme.fn.primaryShade()],
		},
		rules: {
			'.Input': {
				border: `1px solid ${
					theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[4]
				}`,
			},
			'.Input:focus': {
				outlineOffset: '2px',
				outline:
					theme.focusRing === 'always' || theme.focusRing === 'auto'
						? `2px solid ${
								theme.colors[theme.primaryColor][
									theme.colorScheme === 'dark' ? 7 : 5
								]
						  }`
						: 'none',
			},
			'.Input:focus:not(:focus-visible)': {
				outline: theme.focusRing === 'auto' || theme.focusRing === 'never' ? 'none' : '',
			},
			'.Input:focus, .Input:focus-within': {
				outline: 'none',
				borderColor: theme.colors[theme.primaryColor][theme.fn.primaryShade()],
			},
		},
	};
	const options: StripeElementsOptions = {
		clientSecret,
		appearance,
	};

	return (
		<div className='App'>
			{clientSecret && (
				<Elements options={options} stripe={stripePromise}>
					<SubscribeForm />
				</Elements>
			)}
		</div>
	);
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
	const user = await isAuthed(ctx);

	if (!user) {
		return redirect('/subscribe');
	}

	// When they visit this page, get their account and see if it has customer & subscription
	const accountRes = await ssrFetch(api(`accounts/${user.accountId}`), ctx);

	if (!accountRes.ok)
		return {
			props: {
				error: (await accountRes.json())?.message || 'An error ocurred creating customer.',
			},
		};

	const account: Account = await accountRes.json();
	if (!account.stripeId) {
		const customerRes = await ssrFetch(api('payments/customer'), ctx, {
			body: JSON.stringify(account),
			headers: {
				'Content-Type': 'application/json',
			},
			method: 'POST',
		});

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
