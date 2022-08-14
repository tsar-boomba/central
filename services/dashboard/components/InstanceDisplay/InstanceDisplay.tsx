import { useUser } from '@/components/UserProvider';
import { Instance, InstanceStatus } from '@/types/Instance';
import { Role } from '@/types/User';
import { api, callApi, resError } from '@/utils/apiHelpers';
import { requireRole } from '@/utils/authUtils';
import { fetchNotification } from '@/utils/fetchNotification';
import { instanceStatusToIcon, instanceStatusToText } from '@/utils/instances';
import { ActionIcon, Button, Card, Group, Menu, Stack, Text } from '@mantine/core';
import { openConfirmModal } from '@mantine/modals';
import { IconDots, IconDownload, IconExternalLink, IconTrash } from '@tabler/icons';
import { useEffect } from 'react';
import { mutate } from 'swr';

const InstanceDisplay = ({ instance }: { instance: Instance }) => {
	const { user } = useUser();
	const StatusIcon = instanceStatusToIcon(instance.status);
	const statusText = instanceStatusToText(instance.status);

	const updateSwr = () => mutate(api(`accounts/${user?.accountId}/instances`));

	const isAdmin = requireRole(user?.role, Role.Admin);
	const isOk = instance.status === InstanceStatus.Ok;
	const isUnhealthy = instance.status === InstanceStatus.Unhealthy;
	const isDeactivated = instance.status === InstanceStatus.Inactive;

	const deleteInstance = () => {
		const [ok, err] = fetchNotification(`delete-instance-${instance.id}`, {
			message: 'Deleting instance...',
		});
		callApi({ route: `instances/${instance.id}`, method: 'DELETE' }).then(async (res) => {
			if (res.ok) {
				updateSwr();
				return ok({ message: 'Deleted instance. 😁' });
			}
			err({ message: await resError(res.json(), 'Failed to delete instance.') });
		});
	};
	const openDeleteModal = () =>
		openConfirmModal({
			title: 'Are you sure you want to delete this instance?',
			children: (
				<Text>
					Deleting this instance will cause the loss of all data it stores. Export its
					data before deletion if you would like to keep it.
				</Text>
			),
			labels: { confirm: "I'm Sure", cancel: 'Go Back' },
			confirmProps: { color: 'red' },
			onConfirm: deleteInstance,
		});

	// update health
	useEffect(() => {
		// only update is it is ok, unhealthy, or configured
		if (!user || (!isOk && !isUnhealthy && instance.status !== InstanceStatus.Configured))
			return;
		fetch(api(`instances/${instance.id}/health`), { credentials: 'include' }).then(() =>
			updateSwr(),
		);
	}, []);

	return (
		<Card withBorder shadow='sm'>
			<Card.Section withBorder inheritPadding py='xs'>
				<Group position='apart'>
					<Text size={18} weight={700}>
						{instance.name}
					</Text>
					{isAdmin && (
						<Menu withinPortal position='bottom' shadow='sm'>
							<Menu.Target>
								<ActionIcon>
									<IconDots size={16} />
								</ActionIcon>
							</Menu.Target>

							<Menu.Dropdown>
								{!isDeactivated && <Menu.Item>Deactivate</Menu.Item>}
								{isOk && (
									<Menu.Item icon={<IconDownload size={16} />}>
										Export Data
									</Menu.Item>
								)}
								{isAdmin && (
									<Menu.Item
										onClick={
											isOk || isUnhealthy ? openDeleteModal : deleteInstance
										}
										icon={<IconTrash size={16} />}
										color='red'
									>
										Delete
									</Menu.Item>
								)}
							</Menu.Dropdown>
						</Menu>
					)}
				</Group>
			</Card.Section>
			<Card.Section inheritPadding py='md'>
				<Stack align='center'>
					<Stack align='center'>
						<StatusIcon size={64} />
						<Text size={18} weight={500} align='center'>
							{statusText}
						</Text>
					</Stack>
					<Group>
						<Button
							component='a'
							leftIcon={<IconExternalLink size={16} />}
							href={`https://${instance.url}`}
							target='_blank'
						>
							Go to instance
						</Button>
					</Group>
				</Stack>
			</Card.Section>
		</Card>
	);
};

export default InstanceDisplay;
