import { Account } from '@/types/Account';
import { Role } from '@/types/User';
import { RegisterAccount, UpdateAccount } from '@/types/utils';
import { callApi, resError } from '@/utils/apiHelpers';
import { fetchNotification } from '@/utils/fetchNotification';
import { statesAbbr } from '@/utils/states';
import { Box, Button, Divider, Group, Paper, Text } from '@mantine/core';
import { useForm } from '@mantine/form';
import { FormRulesRecord } from '@mantine/form/lib/types';
import { useUser } from '../UserProvider';
import StateInput from './StateInput';
import TextInputInfo from './TextInputInfo';

interface Props {
	account: Account;
}

type FormValues = UpdateAccount;

export const accountValidation: FormRulesRecord<RegisterAccount> = {
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

const AccountForm = ({ account }: Props) => {
	const { user } = useUser();
	const form = useForm<FormValues>({
		initialValues: account,
		validate: accountValidation,
	});

	console.log(form.values);

	const onSubmit = (values: FormValues) => {
		const [ok, err] = fetchNotification('update-account', {
			message: 'Updating your account...',
		});
		callApi({
			route: `accounts/${account.id}`,
			method: 'PUT',
			body: { ...account, ...values },
		}).then(async (res) => {
			if (res.ok) {
				return ok({ message: 'Account successfully updated. 😁' });
			}
			err({
				message: await resError(res.json(), 'Update failed. 😔'),
			});
		});
	};

	return (
		<Box
			component='form'
			style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}
			onSubmit={form.onSubmit(onSubmit)}
		>
			<Paper>
				<TextInputInfo
					required
					disabled={user?.role !== Role.Owner}
					label='Business Name'
					info='Ex: Gamble Logistics LLC'
					{...form.getInputProps('businessName')}
				/>
				<TextInputInfo
					required
					disabled={user?.role !== Role.Owner}
					label='Short Name'
					info='Ex: Gamble Logistics'
					{...form.getInputProps('shortName')}
				/>
				<TextInputInfo
					required
					disabled={user?.role !== Role.Owner}
					placeholder='example@example.com'
					label='Email'
					{...form.getInputProps('email')}
				/>
				<TextInputInfo
					required
					disabled={user?.role !== Role.Owner}
					label='Phone Number'
					{...form.getInputProps('phoneNumber')}
				/>
				<Divider my='md' mx={-8} />
				<Text align='center'>
					This information is used for billing, if you decide to subscribe
				</Text>
				<TextInputInfo
					required
					disabled={user?.role !== Role.Owner}
					placeholder='123 Abc ln'
					label='Address'
					info='Business street address'
					{...form.getInputProps('address')}
				/>
				<Group align='center' grow>
					<TextInputInfo
						required
						disabled={user?.role !== Role.Owner}
						label='City'
						{...form.getInputProps('city')}
					/>
					<StateInput
						required
						disabled={user?.role !== Role.Owner}
						label='State'
						searchable
						{...form.getInputProps('state')}
					/>
				</Group>
				<TextInputInfo
					required
					disabled={user?.role !== Role.Owner}
					label='Zip Code'
					{...form.getInputProps('zipCode')}
				/>
			</Paper>
			<Button type='submit' mt='md'>
				Update Account
			</Button>
		</Box>
	);
};

export default AccountForm;
