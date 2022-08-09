import { useUser } from '@/components/UserProvider';
import { Instance } from '@/types/Instance';
import { api, ssrFetch } from '@/utils/apiHelpers';
import { isAuthed, redirect } from '@/utils/authUtils';
import fetcher from '@/utils/swrFetcher';
import { Loader, Text } from '@mantine/core';
import { GetServerSideProps } from 'next';
import useSWR from 'swr';

interface Props {
	instances: Instance[];
}

const Instances = ({ instances }: Props) => {
	const { user } = useUser();
	const { data, error } = useSWR(
		user ? api(`accounts/${user.accountId}/instances`) : null,
		fetcher,
		{ fallbackData: instances },
	);

	if (!data) return <Loader />;
	if (error) return <Text component='h1'>{error.message || 'An error ocurred.'}</Text>;

	return <div></div>;
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
	const user = await isAuthed(ctx);

	if (user) {
		const instancesRes = await ssrFetch(api(`accounts/${user.accountId}/instances`), ctx);
		if (instancesRes.ok) {
			const instances: Instance[] = await instancesRes.json();

			return {
				props: {
					instances,
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
