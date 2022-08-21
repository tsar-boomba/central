import { Role, User } from '@/types/User';
import { api, callApi } from '@/utils/apiHelpers';
import { requireRole } from '@/utils/authUtils';
import { fetchNotification } from '@/utils/fetchNotification';
import { ActionIcon, Button, Card, Group, Menu, Stack, Text } from '@mantine/core';
import { IconCircleCheck, IconCircleMinus, IconDots } from '@tabler/icons';
import Link from 'next/link';
import { mutate } from 'swr';
import { useUser } from '../UserProvider';

interface Props {
	userData: User;
}

const UserDisplay = ({ userData }: Props) => {
	const { user } = useUser();

	const updateSwr = () => mutate(api(`accounts/${userData.accountId}/users`));

	const isMod = requireRole(user?.role, Role.Moderator);
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
					{isMod && (
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
							</Menu.Dropdown>
						</Menu>
					)}
				</Group>
			</Card.Section>
			{isMod && (
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
