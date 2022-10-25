import { Role, User } from '@/types/User';
import { api, callApi } from '@/utils/apiHelpers';
import { higherRole } from '@/utils/authUtils';
import { fetchNotification } from '@/utils/fetchNotification';
import { ActionIcon, Button, Card, Group, Menu, Stack, Text } from '@mantine/core';
import { openConfirmModal } from '@mantine/modals';
import { IconCircleCheck, IconCircleMinus, IconDots } from '@tabler/icons';
import Link from 'next/link';
import { mutate } from 'swr';
import { useUser } from '@/utils/authUtils';

interface Props {
	userData: User;
}

const UserDisplay = ({ userData }: Props) => {
	const { user, mutate: mutateUser } = useUser();

	const updateSwr = () => mutate(api(`accounts/${userData.accountId}/users`));

	const isHigherRole = higherRole(user?.role, userData.role);
	const isActive = userData.active;

	const toggleStatus = () => {
		const newStatus = userData.active ? 'Inactive' : 'Active';
		const [ok, fail] = fetchNotification(`toggle-status-user-${userData.id}`, {
			message: `Setting user to ${newStatus}...`,
		});
		callApi({ route: `users/${userData.id}/toggle-status`, method: 'PUT' }).then((res) => {
			if (res.ok) {
				ok({
					message: `Successfully set user to ${newStatus}. 😁`,
				});
				updateSwr();
			} else {
				fail({
					message: `Failed to set user to ${newStatus}. 😔`,
				});
			}
		});
	};

	const transferOwner = () => {
		const [ok, fail] = fetchNotification(`toggle-status-user-${userData.id}`, {
			message: `Transferring ownership...`,
		});
		callApi({ route: `users/${userData.id}/transfer-owner`, method: 'PUT' }).then((res) => {
			if (res.ok) {
				ok({
					message: `Successfully transferred ownership. 😁`,
				});
				mutateUser();
				updateSwr();
			} else {
				fail({
					message: `Failed to transferred ownership. 😔`,
				});
			}
		});
	};
	const openTransferOwnerModal = () =>
		openConfirmModal({
			title: 'Are you sure you want to transfer ownership?',
			children: (
				<Text>
					This is irreversible if you don't have access to the user you transfer to. Your
					role will be set to admin if successful.
				</Text>
			),
			labels: { confirm: 'Transfer', cancel: 'Go Back' },
			confirmProps: { color: 'red' },
			onConfirm: transferOwner,
		});

	return (
		<Card withBorder shadow='sm'>
			<Card.Section withBorder inheritPadding py='sm'>
				<Group position='apart' align='start'>
					<Stack spacing={0}>
						<Text size={18} weight={700}>
							{userData.firstName} {userData.lastName}
						</Text>
						<Text>{userData.username}</Text>
					</Stack>
					{(isHigherRole || user?.role === Role.Owner) && (
						<Menu withinPortal position='bottom' shadow='sm'>
							<Menu.Target>
								<ActionIcon>
									<IconDots size={16} />
								</ActionIcon>
							</Menu.Target>

							<Menu.Dropdown>
								<Menu.Item
									icon={
										isActive ? (
											<IconCircleMinus size={16} />
										) : (
											<IconCircleCheck size={16} />
										)
									}
									onClick={toggleStatus}
									disabled={user?.id === userData.id}
								>
									{isActive ? 'Set Inactive' : 'Set Active'}
								</Menu.Item>
								{user?.role === Role.Owner && isHigherRole && (
									<Menu.Item color='red' onClick={openTransferOwnerModal}>
										Transfer Ownership
									</Menu.Item>
								)}
							</Menu.Dropdown>
						</Menu>
					)}
				</Group>
			</Card.Section>
			{(isHigherRole || user?.role === Role.Owner) && (
				<Card.Section withBorder inheritPadding py='sm'>
					<Stack>
						<Link href={`/users/${userData.id}`} passHref>
							<Button component='a'>Edit User</Button>
						</Link>
					</Stack>
				</Card.Section>
			)}
		</Card>
	);
};

export default UserDisplay;
