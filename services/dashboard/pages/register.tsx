import { Button, Group, Paper, Stepper, Text } from '@mantine/core';
import { useForm } from '@mantine/form';
import { FormRulesRecord } from '@mantine/form/lib/types';
import { useState } from 'react';
import TextInputInfo from '../components/Form/TextInputInfo';
import { RegisterAccount, RegisterUser } from '../types/utils';
import { statesAbbr } from '../utils/states';

const NUM_STEPS = 2;

type FormData = { account: RegisterAccount; user: RegisterUser };

const accountValidation: FormRulesRecord<RegisterAccount> = {
	businessName: (v) => (v.length > 0 ? null : 'Business Name cannot be empty.'),
	shortName: (v) => (v.length > 0 ? null : 'Short Name cannot be empty.'),
	email: (v) => (/^\S+@\S+$/.test(v) ? null : 'Invalid email.'),
	address: (v) => (v.length > 1 ? null : 'Address cannot be empty.'),
	city: (v) => (v.length > 0 ? null : 'City cannot be empty.'),
	phoneNumber: (v) =>
		/^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$/.test(v)
			? null
			: 'Invalid US phone number.',
	state: (v) => (statesAbbr.includes(v) ? null : 'Must be a valid state name.'),
	zipCode: (v) => (/[\d]{5}(-[\d]{4})?/.test(v) ? null : 'Zip Code cannot be empty.'),
};

const userValidation: FormRulesRecord<RegisterUser> = {
	firstName: (v) => (v.length > 1 ? null : 'First Name cannot be empty.'),
	lastName: (v) => (v.length > 0 ? null : 'last Name cannot be empty.'),
	password: (v) => (v.length > 0 ? null : 'Password cannot be empty.'),
	username: (v) => (v.length > 0 ? null : 'Username cannot be empty.'),
};

const Register = () => {
	const [active, setActive] = useState(0);

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
			},
		},
		validate: {
			account: active === 0 ? accountValidation : undefined,
			user: active === 1 ? userValidation : undefined,
		},
		validateInputOnChange: true,
	});
	console.log(form.values);

	const nextStep = () =>
		setActive((current) => {
			if (form.validate().hasErrors) {
				return current;
			}
			return current < NUM_STEPS ? current + 1 : current;
		});
	const prevStep = () => setActive((current) => (current > 0 ? current - 1 : current));

	return (
		<>
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
							info='Ex for Gamble Logistics LLC: Gamble Logistics'
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
						<Text align='center' component='h1' sx={{ fontSize: 24 }}>
							Location Info
						</Text>
						<TextInputInfo
							required
							placeholder='123 Abc ln ste 31'
							label='Address'
							info='Business street address'
							{...form.getInputProps('account.address')}
						/>
					</Paper>
				</Stepper.Step>
				<Stepper.Step
					label='Final Step'
					description="Create a user (This is how you'll log in)"
					allowStepSelect={active < NUM_STEPS && active > 1}
				>
					Step 2 content: Create a user
				</Stepper.Step>
				<Stepper.Completed>
					Completed, click back button to get to previous step
				</Stepper.Completed>
			</Stepper>
			<Group position='right' mt='xl'>
				{active > 0 && active < NUM_STEPS && (
					<Button variant='default' onClick={prevStep}>
						Back
					</Button>
				)}
				{active < NUM_STEPS && <Button onClick={nextStep}>Next step</Button>}
			</Group>
		</>
	);
};

export default Register;
