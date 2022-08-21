import { InstanceForm } from '@/components/Form/InstanceForm';
import { Instance } from '@/types/Instance';
import { Role } from '@/types/User';
import { api, isNotFound, isServerError, ssrFetch } from '@/utils/apiHelpers';
import { isAuthed, requireRole } from '@/utils/authUtils';
import fetcher from '@/utils/swrFetcher';
import { Center, Loader, Text } from '@mantine/core';
import { GetServerSideProps } from 'next';
import { useRouter } from 'next/router';
import useSWR from 'swr';

interface Props {
	initialInstance: Instance;
}

const Instance = ({ initialInstance }: Props) => {
	const router = useRouter();
	const { data: instance } = useSWR<Instance>(
		router.query ? api(`instances/${router.query.id}`) : null,
		fetcher,
		{ fallbackData: initialInstance },
	);
	if (!instance)
		return (
			<Center>
				<Loader size='xl' />
			</Center>
		);

	return (
		<div>
			<Text align='center' component='h1' size={36}>
				Edit Instance
			</Text>
			<InstanceForm defaultInstance={instance} id={instance.id} />
		</div>
	);
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
	const user = await isAuthed(ctx);

	if (!user) {
		return {
			redirect: {
				destination: `/login?from=/instances`,
				permanent: false,
			},
		};
	}

	if (!requireRole(user.role, Role.Admin)) {
		return {
			redirect: {
				destination: '/instances',
				permanent: false,
			},
		};
	}

	const res = await ssrFetch(api(`instances/${ctx.params?.id}`), ctx);

	if (isNotFound(res)) {
		return {
			notFound: true,
		};
	}

	if (isServerError(res) || !res.ok) {
		throw new Error('A server error ocurred.');
	}

	const initialInstance = await res.json();
	return {
		props: {
			initialInstance,
		},
	};
};

export default Instance;
