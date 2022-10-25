import { Account } from '@/types/Account';
import { api, callApi } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';
import { Alert, Button, createStyles, Input } from '@mantine/core';
import { showNotification } from '@mantine/notifications';
import { CardElement, useStripe, useElements, CardElementProps } from '@stripe/react-stripe-js';
import { IconAlertCircle } from '@tabler/icons';
import { useRouter } from 'next/router';
import { FormEventHandler, useState } from 'react';
import useSWR from 'swr';
import { useUser } from '@/utils/authUtils';

const sizes = {
	xs: 30,
	sm: 36,
	md: 42,
	lg: 50,
	xl: 60,
};

const size = 'sm';

const useStyles = createStyles((theme) => ({
	card: {
		border: `1px solid ${
			theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[4]
		}`,
		backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.white,
		transition: 'border-color 100ms ease',
		'&:focus, &:focus-within': {
			outline: 'none',
			borderColor: theme.colors[theme.primaryColor][theme.fn.primaryShade()],
		},
		...theme.fn.fontStyles(),
		height: theme.fn.size({ size, sizes }),
		WebkitTapHighlightColor: 'transparent',
		lineHeight: `${theme.fn.size({ size, sizes }) - 2}px`,
		appearance: 'none',
		resize: 'none',
		boxSizing: 'border-box',
		fontSize: theme.fn.size({ size, sizes: theme.fontSizes }),
		minWidth: '300px',
		color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.black,
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'center',
		textAlign: 'left',
		minHeight: theme.fn.size({ size, sizes }),
		paddingLeft: theme.fn.size({ size, sizes }) / 3,
		paddingRight: theme.fn.size({ size, sizes }) / 3,
		borderRadius: theme.fn.radius('sm'),
		...theme.fn.focusStyles(),
	},
}));

const SubscribeForm = () => {
	const { classes, theme } = useStyles();
	const { user } = useUser();
	const { data: account } = useSWR<Account>(
		user?.accountId ? api(`accounts/${user.accountId}`) : null,
		fetcher,
	);
	const router = useRouter();
	const stripe = useStripe();
	const elements = useElements();

	const [message, setMessage] = useState<string | null>(null);
	const [isLoading, setIsLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);

	const successful = () => {
		showNotification({ message: 'Successfully subscribed. Enjoy Milky Web 😁' });
		router.push('/');
	};

	const onChange: CardElementProps['onChange'] = (e) => {
		setError(e.error ? e.error.message : null);
	};

	const handleSubmit: FormEventHandler<HTMLFormElement> = async (e) => {
		e.preventDefault();

		if (!stripe || !elements || !account) {
			// Stripe.js has not yet loaded.
			// Make sure to disable form submission until Stripe.js has loaded.
			return;
		}

		setIsLoading(true);

		const card = elements.getElement('card');

		if (!card) return;

		const { paymentMethod, error } = await stripe.createPaymentMethod({
			type: 'card',
			card,
		});

		if (paymentMethod) {
			return callApi({
				route: 'payments/subscription',
				body: { account, paymentMethodId: paymentMethod.id },
			}).then(async (res) => {
				if (!res.ok) {
					const message = await res
						.json()
						.then((json) => json?.message || 'An api error ocurred.')
						.catch(() => 'An api error ocurred.');
					setMessage(message);
				} else {
					// check for setup
					const sub = await res.json();
					const setupIntent = sub.pending_setup_intent;

					if (setupIntent && setupIntent.status === 'requires_action') {
						return stripe
							.confirmCardSetup(setupIntent.client_secret, {
								payment_method: paymentMethod.id,
							})
							.then((result) => {
								if (result.error) {
									// start code flow to handle updating the payment details
									// Display error message in your UI.
									// The card was declined (i.e. insufficient funds, card has expired, etc)
									setMessage(result.error.message || 'An unknown error ocurred.');
								} else {
									if (result.setupIntent.status === 'succeeded') {
										// There's a risk of the customer closing the window before callback
										// execution. To handle this case, set up a webhook endpoint and
										// listen to setup_intent.succeeded.
										successful();
									}
								}
								setIsLoading(false);
							});
					} else {
						// No customer action needed
						successful();
					}
				}
				setIsLoading(false);
			});
		}

		// This point will only be reached if there is an immediate error when
		// confirming the payment. Otherwise, your customer will be redirected to
		// your `return_url`. For some payment methods like iDEAL, your customer will
		// be redirected to an intermediate site first to authorize the payment, then
		// redirected to the `return_url`.
		if (error?.type === 'card_error' || error?.type === 'validation_error') {
			setMessage(error.message || 'A card/validation error ocurred.');
		} else if (error) {
			setMessage('An unexpected error occurred.');
		}

		setIsLoading(false);
	};

	const cardOptions: CardElementProps['options'] = {
		style: {
			base: {
				color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.black,
			},
		},
	};

	return (
		<form
			id='payment-form'
			style={{
				display: 'flex',
				flexDirection: 'column',
				alignItems: 'center',
				width: '100%',
			}}
			onSubmit={handleSubmit}
		>
			<Input.Wrapper label='Card Info' error={error}>
				<CardElement
					className={classes.card}
					onChange={onChange}
					options={cardOptions}
					id='card-element'
				/>
			</Input.Wrapper>
			<Button
				mt='md'
				loading={isLoading}
				disabled={isLoading || !!error || !account || !stripe || !elements}
				type='submit'
				id='submit'
			>
				Subscribe
			</Button>
			{/* Show any error or success messages */}
			{message && (
				<Alert icon={<IconAlertCircle size={16} />} mt='md' id='payment-message'>
					{message}
				</Alert>
			)}
		</form>
	);
};

export default SubscribeForm;
