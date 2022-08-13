import { api } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';
import { useSubStatus } from '@/utils/useSubStatus';
import {
	Button,
	Group,
	Loader,
	Paper,
	Stack,
	Text,
	ThemeIcon,
	useMantineTheme,
} from '@mantine/core';
import { openConfirmModal } from '@mantine/modals';
import { Elements } from '@stripe/react-stripe-js';
import { loadStripe } from '@stripe/stripe-js';
import useSWR from 'swr';
import GradientCard from '../GradientCard';
import { useUser } from '../UserProvider';
import CardForm from './CardForm';

const stripePromise = loadStripe(process.env.NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY || '');

const ManageSubscription = () => {
	const theme = useMantineTheme();
	const { user } = useUser();
	const { status } = useSubStatus();
	const { data: usage } = useSWR<{ users: number; instances: number }>(
		user ? api(`accounts/${user.accountId}/usage`) : null,
		fetcher,
	);

	if (!user || !usage) return <Loader size='xl' />;

	const openCancelSubModal = () =>
		openConfirmModal({
			title: 'Are you sure you want to cancel your subscription?',
			children: (
				<Text>
					Canceling your subscription will cause all of your instances to change to
					inactive. Please export your data if you want to use it elsewhere.
				</Text>
			),
			labels: { confirm: "I'm Sure", cancel: 'Go Back' },
			onConfirm: () => console.error('Implement this'), // TODO cancel subscriptions
		});
	console.log(status);

	return (
		<Group position='center'>
			<Paper shadow='md' withBorder p='md'>
				<Text mt='0' sx={{ fontSize: 24 }} align='center' component='h2'>
					Your Current Usage
				</Text>
				<Stack align='center'>
					<GradientCard p='md' component='div'>
						<Group noWrap>
							<ThemeIcon size={32} radius='xl' color='white'>
								<Text
									sx={{
										fontSize: 16,
										fontWeight: 900,
										lineHeight: 1,
										width: '100%',
									}}
									align='center'
									color={
										theme.colors[theme.primaryColor][theme.fn.primaryShade()]
									}
								>
									{usage.users}
								</Text>
							</ThemeIcon>
							<Text sx={{ fontSize: 24, fontWeight: 700 }} color='white'>
								{usage.users > 1 ? 'Users' : 'User'}
							</Text>
						</Group>
					</GradientCard>
					<GradientCard p='md' component='div'>
						<Group noWrap>
							<ThemeIcon size={32} radius='xl' color='white'>
								<Text
									sx={{
										fontSize: 16,
										fontWeight: 900,
										lineHeight: 1,
										width: '100%',
									}}
									align='center'
									color={
										theme.colors[theme.primaryColor][theme.fn.primaryShade()]
									}
								>
									{usage.instances}
								</Text>
							</ThemeIcon>
							<Text sx={{ fontSize: 24, fontWeight: 700 }} color='white'>
								{usage.instances > 1 || usage.instances === 0
									? 'Instances'
									: 'Instance'}
							</Text>
						</Group>
					</GradientCard>
					<Text size='sm' sx={{ maxWidth: 300 }}>
						As a reminder, you are charged $10 per user and $100 per instance every
						period (monthly). It is charged based on the maximum number you had this
						period, so the numbers above may not reflect how much you'll actually be
						charged.
					</Text>
				</Stack>
			</Paper>
			{status !== undefined && (
				<Paper shadow='md' withBorder p='md'>
					<Stack align='center'>
						<Text m='0' sx={{ fontSize: 24 }} align='center' component='h2'>
							Manage Your Subscription
						</Text>
						<GradientCard component='div' p='md'>
							<Text m='0' color='white' component='h3' align='center'>
								Change Payment Method
							</Text>
							<Elements stripe={stripePromise}>
								<CardForm />
							</Elements>
						</GradientCard>
						<Button color='red' onClick={openCancelSubModal}>
							Cancel
						</Button>
					</Stack>
				</Paper>
			)}
		</Group>
	);
};

export default ManageSubscription;
