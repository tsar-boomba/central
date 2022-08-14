import { useUser } from '@/components/UserProvider';
import { Instance, InstanceStatus } from '@/types/Instance';
import { Role } from '@/types/User';
import { api, ssrFetch } from '@/utils/apiHelpers';
import { isAuthed, redirect, requireRole } from '@/utils/authUtils';
import { instanceStatusToIcon, instanceStatusToText } from '@/utils/instances';
import fetcher from '@/utils/swrFetcher';
import { useSubStatus } from '@/utils/useSubStatus';
import {
	ActionIcon,
	Button,
	Card,
	Container,
	Group,
	Loader,
	Menu,
	Stack,
	Text,
	TextInput,
} from '@mantine/core';
import { IconDots, IconDownload, IconExternalLink, IconSearch, IconTrash } from '@tabler/icons';
import { GetServerSideProps } from 'next';
import Link from 'next/link';
import useSWR from 'swr';

interface Props {
	initialInstances: Instance[];
}

const InstanceDisplay = ({ instance }: { instance: Instance }) => {
	const { user } = useUser();
	const StatusIcon = instanceStatusToIcon(instance.status);
	const statusText = instanceStatusToText(instance.status);

	const isAdmin = requireRole(user?.role, Role.Admin);
	const isOk = instance.status === InstanceStatus.Ok;
	const isDeactivated = instance.status === InstanceStatus.Inactive;

	return (
		<Card withBorder shadow='sm'>
			<Card.Section withBorder inheritPadding py='xs'>
				<Group position='apart'>
					<Text weight={500}>{instance.name}</Text>
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
								<Menu.Item icon={<IconTrash size={16} />} color='red'>
									Delete
								</Menu.Item>
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

const Instances = ({ initialInstances }: Props) => {
	const { user } = useUser();
	const { isSubbed } = useSubStatus();
	const { data: instances, error } = useSWR<Instance[]>(
		user ? api(`accounts/${user.accountId}/instances`) : null,
		fetcher,
		{ fallbackData: initialInstances },
	);
	const isAdmin = requireRole(user?.role, Role.Admin);

	if (!instances || !user) return <Loader />;
	if (error) return <Text component='h1'>{error.message || 'An error ocurred.'}</Text>;

	return (
		<Container>
			{isAdmin && isSubbed && (
				<Group position='center'>
					<Link href='/instances/create' passHref>
						<Button component='a' color='green'>
							Create Instance
						</Button>
					</Link>
				</Group>
			)}
			<TextInput
				mt='md'
				label='Search'
				placeholder='Search for instances'
				icon={<IconSearch size={16} />}
			/>
			<Group mt='md' position='center'>
				{instances.map((instance) => (
					<InstanceDisplay instance={instance} key={instance.id} />
				))}
			</Group>
		</Container>
	);
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
	const user = await isAuthed(ctx);

	if (user) {
		const instancesRes = await ssrFetch(api(`accounts/${user.accountId}/instances`), ctx);
		if (instancesRes.ok) {
			const initialInstances: Instance[] = await instancesRes.json();

			return {
				props: {
					initialInstances,
				},
			};
		} else {
			if (instancesRes.status < 500)
				return {
					notFound: true,
				};
			throw new Error('Error while fetching instances');
		}
	} else {
		return redirect('instances');
	}
};

export default Instances;
