import { api, ssrFetch } from '@/utils/apiHelpers';
import { Center, Loader } from '@mantine/core';
import { GetServerSideProps } from 'next';

const Health = () => {
	return (
		<Center>
			<Loader size='xl' />
		</Center>
	);
};

export const getServerSideProps: GetServerSideProps = async (ctx) => {
	const crudRes = await ssrFetch(api('verify'), ctx);
	const paymentsRes = await ssrFetch(api('payments'), ctx);

	if (crudRes.status >= 500 || paymentsRes.status >= 500) {
		throw new Error('Unhealthy backend!!!');
	}

	return {
		redirect: {
			destination: '/',
			permanent: false,
		},
	};
};

export default Health;
