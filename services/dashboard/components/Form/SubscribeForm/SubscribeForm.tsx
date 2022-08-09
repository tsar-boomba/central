import { Account } from '@/types/Account';
import { api, callApi } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';
import { Button, createStyles } from '@mantine/core';
import { CardElement, useStripe, useElements, CardElementProps } from '@stripe/react-stripe-js';
import { FormEventHandler, useState } from 'react';
import useSWR from 'swr';
import { useUser } from '../../UserProvider';

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
	const stripe = useStripe();
	const elements = useElements();

	const [message, setMessage] = useState<string | null>(null);
	const [isLoading, setIsLoading] = useState(false);

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
				route: 'payments/subscribe',
				body: { account, paymentMethodId: paymentMethod.id },
			});
		}

		// This point will only be reached if there is an immediate error when
		// confirming the payment. Otherwise, your customer will be redirected to
		// your `return_url`. For some payment methods like iDEAL, your customer will
		// be redirected to an intermediate site first to authorize the payment, then
		// redirected to the `return_url`.
		if (error?.type === 'card_error' || error?.type === 'validation_error') {
			setMessage(error.message || 'A card/validation error ocurred.');
		} else {
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
			<CardElement className={classes.card} options={cardOptions} id='card-element' />
			<Button
				mt='md'
				loading={isLoading}
				disabled={isLoading || !account || !stripe || !elements}
				type='submit'
				id='submit'
			>
				Subscribe
			</Button>
			{/* Show any error or success messages */}
			{message && <div id='payment-message'>{message}</div>}
		</form>
	);
};

export default SubscribeForm;
