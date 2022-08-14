import { InstanceForm } from '@/components/Form/InstanceForm';
import { Role } from '@/types/User';
import { DEFAULT_SSR } from '@/utils/authUtils';
import { Text } from '@mantine/core';

const CreateInstance = () => {
	return (
		<div>
			<Text align='center' component='h1' size={36}>
				Create an Instance
			</Text>
			<InstanceForm create />
		</div>
	);
};

export const getServerSideProps = DEFAULT_SSR('/create', Role.Admin);

export default CreateInstance;
