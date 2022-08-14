import { InstanceDisplay } from '@/components/InstanceDisplay';
import { useUser } from '@/components/UserProvider';
import { Instance } from '@/types/Instance';
import { Role } from '@/types/User';
import { api, ssrFetch } from '@/utils/apiHelpers';
import { isAuthed, redirect, requireRole } from '@/utils/authUtils';
import fetcher from '@/utils/swrFetcher';
import { useSubStatus } from '@/utils/useSubStatus';
import { Button, Container, Group, Loader, Text, TextInput } from '@mantine/core';
import { IconSearch } from '@tabler/icons';
import { GetServerSideProps } from 'next';
import Link from 'next/link';
import useSWR from 'swr';

interface Props {
	initialInstances: Instance[];
}

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
