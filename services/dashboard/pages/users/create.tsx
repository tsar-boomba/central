import { UserForm } from '@/components/Form/UserForm';
import { Role } from '@/types/User';
import { DEFAULT_SSR } from '@/utils/authUtils';
import { Text } from '@mantine/core';

const CreateUser = () => {
	return (
		<div>
			<Text align='center' component='h1' size={36}>
				Create User
			</Text>
			<UserForm create />
		</div>
	);
};

export const getServerSideProps = DEFAULT_SSR('/users', Role.Moderator);

export default CreateUser;
