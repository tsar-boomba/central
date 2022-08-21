import { useUser } from '@/components/UserProvider';
import { Instance, InstanceStatus } from '@/types/Instance';
import { Role } from '@/types/User';
import { api, callApi, resError } from '@/utils/apiHelpers';
import { requireRole } from '@/utils/authUtils';
import { fetchNotification } from '@/utils/fetchNotification';
import { instanceStatusToIcon, instanceStatusToText } from '@/utils/instances';
import { ActionIcon, Button, Card, Group, Menu, Stack, Text } from '@mantine/core';
import { openConfirmModal } from '@mantine/modals';
import {
	IconCircleMinus,
	IconDots,
	IconDownload,
	IconExternalLink,
	IconRocket,
	IconTrash,
} from '@tabler/icons';
import Link from 'next/link';
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
	const isFailed = instance.status === InstanceStatus.Failed;
	const isDeploying = instance.status === InstanceStatus.Deploying;
	const isConfigured = instance.status === InstanceStatus.Configured;

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

	const deactivateInstance = () => {
		const [ok, err] = fetchNotification(`deactivate-instance-${instance.id}`, {
			message: 'Deactivating instance...',
		});
		callApi({ route: `instances/${instance.id}/deactivate`, method: 'PUT' }).then(
			async (res) => {
				if (res.ok) {
					updateSwr();
					return ok({ message: 'Deactivated instance. 😁' });
				}
				err({ message: await resError(res.json(), 'Failed to deactivate instance.') });
			},
		);
	};
	const openDeactivateModal = () =>
		openConfirmModal({
			title: 'Are you sure you want to deactivate this instance?',
			children: (
				<Text>
					Deactivating this instance will cause the loss of all data it stores. Export its
					data before deactivating if you would like to keep it. The record will remain
					here and you can easily deploy it again later.
				</Text>
			),
			labels: { confirm: "I'm Sure", cancel: 'Go Back' },
			confirmProps: { color: 'red' },
			onConfirm: deactivateInstance,
		});

	const deployInstance = () => {
		const [ok, err] = fetchNotification(`deploy-instance-${instance.id}`, {
			message: 'Starting deployment...',
			autoClose: 15000,
		});
		callApi({ route: `instances/${instance.id}/deploy`, method: 'PUT' }).then(async (res) => {
			if (res.ok) {
				updateSwr();
				return ok({ message: 'Started deploying instance. 😁' });
			}
			err({ message: await resError(res.json(), 'Failed to start instance deployment. 😔') });
		});
	};
	const openDeployModal = () =>
		openConfirmModal({
			title: 'Are you sure you want to deploy this instance?',
			children: (
				<Text>
					If deployment is successful it will be added to your current usage for the
					month.
				</Text>
			),
			labels: { confirm: 'Deploy', cancel: 'Go Back' },
			confirmProps: { color: 'green' },
			onConfirm: deployInstance,
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
					{isAdmin && !(isDeploying || isConfigured) && (
						<Menu withinPortal position='bottom' shadow='sm'>
							<Menu.Target>
								<ActionIcon>
									<IconDots size={16} />
								</ActionIcon>
							</Menu.Target>

							<Menu.Dropdown>
								{isAdmin && (isFailed || isDeactivated) && (
									<Menu.Item
										icon={<IconRocket size={16} />}
										onClick={openDeployModal}
									>
										Deploy
									</Menu.Item>
								)}
								{isAdmin && (isOk || isUnhealthy) && (
									<Menu.Item
										icon={<IconCircleMinus size={16} />}
										onClick={openDeactivateModal}
									>
										Deactivate
									</Menu.Item>
								)}
								{isAdmin && isOk && (
									<Menu.Item icon={<IconDownload size={16} />}>
										Export Data
									</Menu.Item>
								)}
								{isAdmin && !isDeploying && !isConfigured && (
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
						<StatusIcon size={48} />
						<Text size={18} weight={500} align='center'>
							{statusText}
						</Text>
					</Stack>
					<Stack>
						{(isOk || isUnhealthy) && (
							<Button
								component='a'
								leftIcon={<IconExternalLink size={16} />}
								href={`https://${instance.url}`}
								target='_blank'
							>
								Go to instance
							</Button>
						)}
						{isAdmin && (
							<Link href={`/instances/${instance.id}`} passHref>
								<Button component='a'>Edit Instance</Button>
							</Link>
						)}
					</Stack>
				</Stack>
			</Card.Section>
		</Card>
	);
};

export default InstanceDisplay;
