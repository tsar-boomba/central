import { UserDisplay } from '@/components/UserDisplay';
import { useUser } from '@/utils/authUtils';
import { Role, User } from '@/types/User';
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
	initialUsers: User[];
}

const Users = ({ initialUsers }: Props) => {
	const { user } = useUser();
	const { isSubbed } = useSubStatus();
	const { data: users, error } = useSWR<User[]>(
		user ? api(`accounts/${user.accountId}/users`) : null,
		fetcher,
		{ fallbackData: initialUsers },
	);
	const isMod = requireRole(user?.role, Role.Moderator);

	if (!users || !user) return <Loader />;
	if (error) return <Text component='h1'>{error.message || 'An error ocurred.'}</Text>;

	return (
		<Container>
			{isMod && isSubbed && (
				<Group position='center'>
					<Link href='/users/create' passHref>
						<Button component='a' color='green'>
							Create user
						</Button>
					</Link>
				</Group>
			)}
			<TextInput
				mt='md'
				label='Search'
				placeholder='Search for users'
				icon={<IconSearch size={16} />}
			/>
			<Group mt='md' position='center'>
				{users.map((user) => (
					<UserDisplay key={user.id} userData={user} />
				))}
			</Group>
		</Container>
	);
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
	const user = await isAuthed(ctx);

	if (user) {
		const res = await ssrFetch(api(`accounts/${user.accountId}/users`), ctx);
		if (res.ok) {
			const initialUsers: User[] = await res.json();

			return {
				props: {
					initialUsers,
				},
			};
		} else {
			if (res.status < 500)
				return {
					notFound: true,
				};
			throw new Error('Error while fetching instances');
		}
	} else {
		return redirect('instances');
	}
};

export default Users;
