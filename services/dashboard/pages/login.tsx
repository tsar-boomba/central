import { useUser } from '@/components/UserProvider';
import { callApi } from '@/utils/apiHelpers';
import { fetchNotification } from '@/utils/fetchNotification';
import { Anchor, Button, Group, Paper, PasswordInput, Text, TextInput } from '@mantine/core';
import { setCookie } from 'ez-cookies';
import { GetServerSideProps, NextPage } from 'next';
import { useRouter } from 'next/router';
import { useForm } from 'react-hook-form';

interface Props {
	accountId: string | null;
}

interface FormValues {
	username: string;
	password: string;
}

const Login: NextPage<Props> = ({ accountId }) => {
	const { mutate } = useUser();
	const router = useRouter();
	const { register, handleSubmit } = useForm<FormValues>();

	const onSubmit = (values: FormValues) => {
		const [success, fail] = fetchNotification('login');
		callApi({ route: 'login', body: { accountId, ...values } }).then(async (res) => {
			if (res.ok) {
				setCookie('account', accountId || '', { maxAge: 60 * 60 * 24 * 365 * 10 });
				success({ message: 'Logged in 😀.' });
				await mutate();
				router.push(String(router.query.from || '/'));
			} else {
				const json = await res.json();
				fail({ message: json?.message });
			}
		});
	};

	return (
		<>
			{accountId ? (
				<>
					<Text mt='lg' style={{ fontSize: 40, fontWeight: 700 }}>
						Welcome Back!
					</Text>
					<Paper withBorder shadow='md' p='xl' m='lg' style={{ maxWidth: 240 }}>
						<form onSubmit={handleSubmit(onSubmit)}>
							<TextInput
								label='Username'
								{...register('username', { required: true })}
							/>
							<PasswordInput
								label='Password'
								{...register('password', { required: true })}
							/>
							<Group mt='md' position='right'>
								<Anchor>Forgot password?</Anchor>
							</Group>
							<Button mt='md' fullWidth type='submit'>
								Submit
							</Button>
						</form>
					</Paper>
				</>
			) : (
				<h1>Invalid link! Use one provided by the company.</h1>
			)}
		</>
	);
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => ({
	props: { accountId: String(ctx.query.account ?? '') || ctx.req.cookies.account || null },
});

export default Login;
