import { UserForm } from '@/components/Form/UserForm';
import { Role, User } from '@/types/User';
import { api, isNotFound, isServerError, ssrFetch } from '@/utils/apiHelpers';
import { higherRole, isAuthed, requireRole } from '@/utils/authUtils';
import fetcher from '@/utils/swrFetcher';
import { Center, Loader, Text } from '@mantine/core';
import { GetServerSideProps } from 'next';
import { useRouter } from 'next/router';
import useSWR from 'swr';

interface Props {
	initialUser: User;
}

const Instance = ({ initialUser }: Props) => {
	const router = useRouter();
	const { data: user } = useSWR<User>(
		router.query ? api(`users/${router.query.id}`) : null,
		fetcher,
		{ fallbackData: initialUser },
	);
	if (!user)
		return (
			<Center>
				<Loader size='xl' />
			</Center>
		);
	return (
		<div>
			<Text align='center' component='h1' size={36}>
				Edit User
			</Text>
			<UserForm defaultUser={user} id={user.id} />
		</div>
	);
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
	const user = await isAuthed(ctx);

	if (!user) {
		return {
			redirect: {
				destination: `/login?from=/users`,
				permanent: false,
			},
		};
	}

	if (!requireRole(user.role, Role.Moderator)) {
		return {
			redirect: {
				destination: '/users',
				permanent: false,
			},
		};
	}

	const res = await ssrFetch(api(`users/${ctx.params?.id}`), ctx);

	if (isNotFound(res)) {
		return {
			notFound: true,
		};
	}

	if (isServerError(res) || !res.ok) {
		throw new Error('A server error ocurred.');
	}

	const initialUser: User = await res.json();

	if (user.role !== Role.Owner && !higherRole(user.role, initialUser.role)) {
		return {
			redirect: {
				destination: `/users`,
				permanent: false,
			},
		};
	}

	return {
		props: {
			initialUser,
		},
	};
};

export default Instance;
