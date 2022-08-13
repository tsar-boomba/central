import { useUser } from '@/components/UserProvider';
import { Instance } from '@/types/Instance';
import { api, ssrFetch } from '@/utils/apiHelpers';
import { isAuthed, redirect } from '@/utils/authUtils';
import fetcher from '@/utils/swrFetcher';
import { ActionIcon, Card, Container, Group, Loader, Menu, Text } from '@mantine/core';
import { IconDots } from '@tabler/icons';
import { GetServerSideProps } from 'next';
import useSWR from 'swr';

interface Props {
	initialInstances: Instance[];
}

const InstanceDisplay = ({ instance }: { instance: Instance }) => {
	return (
		<Card withBorder shadow='sm'>
			<Card.Section withBorder inheritPadding py='xs'>
				<Group position='apart'>
					<Text weight={500}>{instance.name}</Text>
					<Menu position='bottom-end' shadow='sm'>
						<Menu.Target>
							<ActionIcon>
								<IconDots size={16} />
							</ActionIcon>
						</Menu.Target>

						<Menu.Dropdown>
							<Menu.Item>Download zip</Menu.Item>
							<Menu.Item>Preview all</Menu.Item>
							<Menu.Item color='red'>Delete</Menu.Item>
						</Menu.Dropdown>
					</Menu>
				</Group>
			</Card.Section>
		</Card>
	);
};

const Instances = ({ initialInstances }: Props) => {
	const { user } = useUser();
	const { data: instances, error } = useSWR<Instance[]>(
		user ? api(`accounts/${user.accountId}/instances`) : null,
		fetcher,
		{ fallbackData: initialInstances },
	);

	if (!instances) return <Loader />;
	if (error) return <Text component='h1'>{error.message || 'An error ocurred.'}</Text>;

	return (
		<Container>
			<Group>
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
