import { Account } from '@/types/Account';
import { api, callApi } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';
import { Alert, Button, createStyles, Input } from '@mantine/core';
import { showNotification } from '@mantine/notifications';
import { CardElement, useStripe, useElements, CardElementProps } from '@stripe/react-stripe-js';
import { IconAlertCircle } from '@tabler/icons';
import { FormEventHandler, useState } from 'react';
import useSWR from 'swr';
import { useUser } from '../UserProvider';

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

const CardForm = () => {
	const { classes, theme } = useStyles();
	const { user } = useUser();
	const { data: account } = useSWR<Account>(
		user?.accountId ? api(`accounts/${user.accountId}`) : null,
		fetcher,
	);
	const stripe = useStripe();
	const elements = useElements();

	const [message, setMessage] = useState<string | null>(null);
	const [isLoading, setIsLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);

	const success = () => {
		showNotification({ message: 'Successfully updated payment method. 😁' });
		setMessage(null);
		setError(null);
		setIsLoading(false);
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
			callApi({
				route: 'payments/subscription',
				body: { account, paymentMethodId: paymentMethod.id },
				method: 'PUT',
			}).then(async (res) => {
				if (!res.ok) {
					const message = await res
						.json()
						.then((json) => json?.message || 'An api error ocurred.')
						.catch(() => 'An api error ocurred.');
					return setMessage(message);
				}

				type UpdateSubRes = {
					invoice?: {
						payment_intent?: {
							client_secret: string;
							status: 'requires_action' | 'requires_payment_method';
						};
					};
				};

				const { invoice }: UpdateSubRes = await res.json();

				console.log('invoice:', invoice);
				if (invoice) {
					// retry payment
					const paymentIntent = invoice.payment_intent;

					if (!paymentIntent) return;

					if (
						paymentIntent.status === 'requires_action' ||
						paymentIntent.status === 'requires_payment_method'
					) {
						return stripe
							.confirmCardPayment(paymentIntent.client_secret, {
								payment_method: paymentMethod.id,
							})
							.then((result) => {
								if (result.error) {
									setMessage(
										result.error.message ||
											'An error ocurred with your payment method.',
									);
								} else {
									if (result.paymentIntent.status === 'succeeded') {
										// There's a risk of the customer closing the window before callback
										// execution. To handle this case, set up a webhook endpoint and
										// listen to invoice.paid. This webhook endpoint returns an Invoice.
										success();
									}
								}
								setIsLoading(false);
							});
					}
				}

				success();
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
			<Input.Wrapper label='New Card Info' error={error}>
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
				disabled={isLoading || !account || !stripe || !elements}
				type='submit'
				id='submit'
			>
				Update
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

export default CardForm;
