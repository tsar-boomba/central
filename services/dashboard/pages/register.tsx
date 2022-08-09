import {
	Box,
	Button,
	Divider,
	Group,
	Loader,
	Paper,
	PasswordInput,
	Progress,
	Stack,
	Stepper,
	Text,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { FormRulesRecord } from '@mantine/form/lib/types';
import { showNotification } from '@mantine/notifications';
import { setCookie } from 'ez-cookies';
import { useRouter } from 'next/router';
import { useState } from 'react';
import { CgCheck, CgClose } from '@tabler/icons';
import StateInput from '../components/Form/StateInput';
import TextInputInfo from '../components/Form/TextInputInfo';
import { Account } from '../types/Account';
import { Resource, Role, User } from '../types/User';
import { NewAccount, NewUser, RegisterAccount, RegisterUser } from '../types/utils';
import { callApi } from '../utils/apiHelpers';
import { statesAbbr } from '../utils/states';

const NUM_STEPS = 2;

type FormData = { account: RegisterAccount; user: RegisterUser };

const PasswordRequirement = ({ meets, label }: { meets: boolean; label: string }) => {
	return (
		<Text
			color={meets ? 'green' : 'red'}
			sx={{ display: 'flex', alignItems: 'center' }}
			mt={7}
			size='sm'
		>
			{meets ? <CgCheck size={14} /> : <CgClose size={14} />}{' '}
			<Box ml={10} sx={{ fontWeight: 500 }}>
				{label}
			</Box>
		</Text>
	);
};

const requirements = [
	{ re: /[0-9]/, label: 'Includes number' },
	{ re: /[a-z]/, label: 'Includes lowercase letter' },
	{ re: /[A-Z]/, label: 'Includes uppercase letter' },
	{ re: /[$&+,:;=?@#|'<>.^*()%!-]/, label: 'Includes special symbol' },
];

const getStrength = (password: string) => {
	let multiplier = password.length > 5 ? 0 : 1;

	requirements.forEach((requirement) => {
		if (!requirement.re.test(password)) {
			multiplier += 1;
		}
	});

	return Math.max(100 - (100 / (requirements.length + 1)) * multiplier, 10);
};

const accountValidation: FormRulesRecord<RegisterAccount> = {
	businessName: (v) => (v.length > 0 ? null : 'Business Name cannot be empty.'),
	shortName: (v) => (v.length > 0 ? null : 'Short Name cannot be empty.'),
	email: (v) => (/^[^\s@]+@([^\s@.,]+\.)+[^\s@.,]{2,}$/.test(v) ? null : 'Invalid email.'),
	phoneNumber: (v) =>
		/^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$/.test(v)
			? null
			: 'Invalid US phone number.',
	address: (v) => (v.length > 0 ? null : 'Address cannot be empty.'),
	city: (v) => (v.length > 0 ? null : 'City cannot be empty.'),
	state: (v) => (statesAbbr.includes(v) ? null : 'Must be a valid state name.'),
	zipCode: (v) => (/[\d]{5}(-[\d]{4})?/.test(v) ? null : 'Zip Code cannot be empty.'),
};

const userValidation: FormRulesRecord<RegisterUser> = {
	username: (v) => (v.length > 0 ? null : 'Username cannot be empty.'),
	firstName: (v) => (v.length > 1 ? null : 'First Name cannot be empty.'),
	lastName: (v) => (v.length > 0 ? null : 'Last Name cannot be empty.'),
	password: (v) => (getStrength(v) >= 100 ? null : 'Password must meet requirements.'),
	confirmPass: (v, values: any) => (v === values.user.password ? null : 'Passwords must match.'),
};

const Register = () => {
	const router = useRouter();
	const [active, setActive] = useState(0);
	const [error, setError] = useState('');
	const form = useForm<FormData>({
		initialValues: {
			account: {
				address: '',
				businessName: '',
				city: '',
				email: '',
				phoneNumber: '',
				shortName: '',
				state: '',
				zipCode: '',
			},
			user: {
				firstName: '',
				lastName: '',
				password: '',
				username: '',
				confirmPass: '',
			},
		},
		validate: {
			account: active === 0 ? accountValidation : undefined,
			user: active === 1 ? userValidation : undefined,
		},
	});

	// make sure password is strong
	const checks = requirements.map((requirement, index) => (
		<PasswordRequirement
			key={index}
			label={requirement.label}
			meets={requirement.re.test(form.values.user.password)}
		/>
	));
	const strength = getStrength(form.values.user.password);
	const color = strength === 100 ? 'green' : strength > 50 ? 'yellow' : 'red';

	const nextStep = () =>
		setActive((current) => {
			if (form.validate().hasErrors) return current;
			return current < NUM_STEPS ? current + 1 : current;
		});
	const prevStep = () => setActive((current) => (current > 0 ? current - 1 : current));

	const onSubmit = (values: FormData) => {
		nextStep();
		// active isn't updated yet
		if (active + 1 < NUM_STEPS) return;
		const account: NewAccount = {
			...values.account,
		};
		const user: NewUser = {
			active: true,
			createPerms: [Resource.Carrier, Resource.Shipper, Resource.Load],
			deletePerms: [Resource.Carrier, Resource.Shipper, Resource.Load],
			updatePerms: [Resource.Carrier, Resource.Shipper, Resource.Load],
			instances: [],
			role: Role.Owner,
			...values.user,
		};
		callApi({ route: 'register', body: { account, user } }).then(async (res) => {
			if (res.ok) {
				// If successful, log them in
				const json: { account: Account; user: User } = await res.json();
				return callApi({
					route: 'login',
					body: {
						accountId: json.account.id,
						username: user.username,
						password: values.user.password,
					},
				}).then((res) => {
					if (res.ok) {
						setCookie('account', json.account.id);
						showNotification({ message: 'Welcome to Milky Web 😁!' });
						router.push('/');
					} else {
						router.push(`/login?account=${json.account.id}`);
					}
				});
			}
			// error ocurred
			setError(
				await res
					.json()
					.then((json) => json?.message || 'An error ocurred.')
					.catch(() => 'An error ocurred.'),
			);
		});
	};

	return (
		<Box
			component='form'
			sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}
			onSubmit={form.onSubmit(onSubmit)}
		>
			<Text mt={0} component='h1' sx={{ fontSize: 36 }}>
				Register
			</Text>
			<Stepper active={active} onStepClick={setActive} breakpoint='xs'>
				<Stepper.Step
					label='First Step'
					description='Add account data'
					allowStepSelect={active < NUM_STEPS && active > 0}
				>
					<Paper>
						<Text align='center' component='h1' sx={{ fontSize: 24 }}>
							Create Account
						</Text>
						<TextInputInfo
							required
							label='Business Name'
							info='Ex: Gamble Logistics LLC'
							{...form.getInputProps('account.businessName')}
						/>
						<TextInputInfo
							required
							label='Short Name'
							info='Ex: Gamble Logistics'
							{...form.getInputProps('account.shortName')}
						/>
						<TextInputInfo
							required
							placeholder='example@example.com'
							label='Email'
							{...form.getInputProps('account.email')}
						/>
						<TextInputInfo
							required
							label='Phone Number'
							{...form.getInputProps('account.phoneNumber')}
						/>
						<Divider my='md' mx={-8} />
						<Text align='center'>
							This information is used for billing, if you decide to subscribe
						</Text>
						<TextInputInfo
							required
							placeholder='123 Abc ln'
							label='Address'
							info='Business street address'
							{...form.getInputProps('account.address')}
						/>
						<Group align='center' grow>
							<TextInputInfo
								required
								label='City'
								{...form.getInputProps('account.city')}
							/>
							<StateInput
								required
								label='State'
								searchable
								{...form.getInputProps('account.state')}
							/>
						</Group>
						<TextInputInfo
							required
							label='Zip Code'
							{...form.getInputProps('account.zipCode')}
						/>
					</Paper>
				</Stepper.Step>
				<Stepper.Step
					label='Final Step'
					description="Create a user (This is how you'll log in)"
					allowStepSelect={active < NUM_STEPS && active > 1}
				>
					<TextInputInfo
						required
						label='Username'
						{...form.getInputProps('user.username')}
					/>
					<Group align='center' grow>
						<TextInputInfo
							required
							label='First Name'
							{...form.getInputProps('user.firstName')}
						/>
						<TextInputInfo
							required
							label='Last Name'
							{...form.getInputProps('user.lastName')}
						/>
					</Group>
					<Group grow>
						<PasswordInput
							required
							label='Password'
							{...form.getInputProps('user.password')}
						/>
						<PasswordInput
							required
							label='Confirm Password'
							{...form.getInputProps('user.confirmPass')}
						/>
					</Group>
					<Box mt='md'>
						<Progress color={color} value={strength} size={5} mb='md' />
						<PasswordRequirement
							label='Includes at least 6 characters'
							meets={form.values.user.password.length > 5}
						/>
						{checks}
					</Box>
				</Stepper.Step>
				<Stepper.Completed>
					{!error ? (
						<Stack align='center'>
							<Loader />
							<Text>Creating account...</Text>
						</Stack>
					) : (
						<Box>
							<Text>{error}</Text>
						</Box>
					)}
				</Stepper.Completed>
			</Stepper>
			<Group position='right' mt='xl'>
				{active > 0 && active < NUM_STEPS && (
					<Button type='button' variant='default' onClick={prevStep}>
						Back
					</Button>
				)}
				{active < NUM_STEPS && (
					<Button type='submit'>
						{active + 1 === NUM_STEPS ? 'Submit' : 'Next step'}
					</Button>
				)}
			</Group>
		</Box>
	);
};

export default Register;
