import { InstanceForm } from '@/components/Form/InstanceForm';
import { Instance } from '@/types/Instance';
import { Role } from '@/types/User';
import { api, isNotFound, isServerError, ssrFetch } from '@/utils/apiHelpers';
import { isAuthed, requireRole } from '@/utils/authUtils';
import { Text } from '@mantine/core';
import { GetServerSideProps } from 'next';

interface Props {
	instance: Instance;
}

const Instance = ({ instance }: Props) => {
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
				destination: 'instances',
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

	const instance = await res.json();
	return {
		props: {
			instance,
		},
	};
};

export default Instance;
