import AccountForm from '@/components/Form/AccountForm';
import { ManageSubscription } from '@/components/ManageSubscription';
import { useAccount, useUser } from '@/components/UserProvider';
import { Role } from '@/types/User';
import { requireRole } from '@/utils/authUtils';
import { Loader, Stack, Tabs, Text } from '@mantine/core';
import { IconChartBar, IconCoin } from '@tabler/icons';

const Account = () => {
	const { user } = useUser();
	const { account } = useAccount();
	const isOwner = requireRole(user?.role, Role.Owner);

	return (
		<Tabs sx={{ width: '100%' }} defaultValue='data'>
			<Tabs.List>
				<Tabs.Tab value='data' icon={<IconChartBar size={14} />}>
					Data
				</Tabs.Tab>
				{isOwner && (
					<Tabs.Tab value='subscription' icon={<IconCoin size={14} />}>
						Subscription
					</Tabs.Tab>
				)}
			</Tabs.List>

			<Tabs.Panel value='data' pt='xs'>
				<Stack align='center'>
					<Text sx={{ fontSize: 36 }} align='center' component='h1'>
						{isOwner && 'Update Your'} Account Data
					</Text>
					{account ? <AccountForm account={account} /> : <Loader size='xl' />}
				</Stack>
			</Tabs.Panel>

			{isOwner && (
				<Tabs.Panel value='subscription' pt='xs' pb='xl'>
					<Stack align='center'>
						<Text sx={{ fontSize: 36 }} align='center' component='h1'>
							Manage your subscription
						</Text>
						<ManageSubscription />
					</Stack>
				</Tabs.Panel>
			)}
		</Tabs>
	);
};

export default Account;
