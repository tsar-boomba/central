import { Button, Group, Stepper } from '@mantine/core';
import { useForm } from '@mantine/form';
import { useState } from 'react';
import { RegisterAccount, RegisterUser } from '../types/utils';

const NUM_STEPS = 2;

const Register = () => {
	const [active, setActive] = useState(0);

	const form = useForm<{ account: RegisterAccount; user: RegisterUser }>({
		validate: {
			account:
				active === 0
					? {
							address: (v) =>
								v.length > 1 ? null : 'Address length must be greater than 1!',
					  }
					: undefined,
			user: active === 1 ? {} : undefined,
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
			<Stepper active={active} onStepClick={setActive} breakpoint='sm'>
				<Stepper.Step
					label='First Step'
					description='Add account data'
					allowStepSelect={active > 0}
				>
					Step 1 content: Create an account
				</Stepper.Step>
				<Stepper.Step
					label='Final Step'
					description="Create a user (This is how you'll log in)"
					allowStepSelect={active > 1}
				>
					Step 2 content: Create a user
				</Stepper.Step>
				<Stepper.Completed>
					Completed, click back button to get to previous step
				</Stepper.Completed>
			</Stepper>
			<Group position='right' mt='xl'>
				{active > 0 && active < NUM_STEPS && (
					<Button
						disabled={Object.keys(form.errors).length > 0}
						variant='default'
						onClick={prevStep}
					>
						Back
					</Button>
				)}
				{active < NUM_STEPS && (
					<Button disabled={Object.keys(form.errors).length > 0} onClick={nextStep}>
						Next step
					</Button>
				)}
			</Group>
		</>
	);
};

export default Register;
