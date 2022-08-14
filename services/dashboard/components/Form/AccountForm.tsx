import { Account } from '@/types/Account';
import { Role } from '@/types/User';
import { RegisterAccount, UpdateAccount } from '@/types/utils';
import { callApi, resError } from '@/utils/apiHelpers';
import { requireRole } from '@/utils/authUtils';
import { fetchNotification } from '@/utils/fetchNotification';
import { statesAbbr } from '@/utils/states';
import { Box, Button, Divider, Group, Paper, Text, TextInput } from '@mantine/core';
import { useForm } from '@mantine/form';
import { FormRulesRecord } from '@mantine/form/lib/types';
import { useState } from 'react';
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
	address1: (v) => (v.length > 0 ? null : 'Address cannot be empty.'),
	address2: (v: string | null) =>
		v !== null ? (v.length > 0 ? null : 'Address cannot be empty.') : null,
	city: (v) => (v.length > 0 ? null : 'City cannot be empty.'),
	state: (v) => (statesAbbr.includes(v) ? null : 'Must be a valid state name.'),
	zipCode: (v) => (/[\d]{5}(-[\d]{4})?/.test(v) ? null : 'Zip Code cannot be empty.'),
};

const AccountForm = ({ account }: Props) => {
	const { user } = useUser();
	const [submitting, setSubmitting] = useState(false);
	const form = useForm<FormValues>({
		initialValues: account,
		validate: accountValidation,
	});
	const isOwner = requireRole(user?.role, Role.Owner);

	const onSubmit = (values: FormValues) => {
		setSubmitting(true); // disable submit and show loader
		const [ok, err] = fetchNotification('update-account', {
			message: 'Updating your account...',
		});
		callApi({
			route: `accounts/${account.id}`,
			method: 'PUT',
			body: {
				...account,
				...values,
				address2: account.address2 === '' ? null : account.address2,
			},
		})
			.then(async (res) => {
				if (res.ok) {
					return ok({ message: 'Account successfully updated. 😁' });
				}
				err({
					message: await resError(res.json(), 'Update failed. 😔'),
				});
			})
			.finally(() => setSubmitting(false));
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
					disabled={!isOwner}
					label='Business Name'
					info='Ex: Gamble Logistics LLC'
					{...form.getInputProps('businessName')}
				/>
				<TextInputInfo
					required
					disabled={!isOwner}
					label='Short Name'
					info='Ex: Gamble Logistics'
					{...form.getInputProps('shortName')}
				/>
				<TextInputInfo
					required
					disabled={!isOwner}
					placeholder='example@example.com'
					label='Email'
					{...form.getInputProps('email')}
				/>
				<TextInputInfo
					required
					disabled={!isOwner}
					label='Phone Number'
					{...form.getInputProps('phoneNumber')}
				/>
				<Divider my='md' />
				<Text align='center'>
					This information is used for billing, if you decide to subscribe
				</Text>
				<TextInput
					required
					placeholder='123 Abc ln'
					label='Address Line 1'
					{...form.getInputProps('address1')}
				/>
				<TextInput
					placeholder='ste 512'
					label='Address Line 2'
					{...form.getInputProps('address2')}
					onChange={(e) => {
						// if empty string, it will be set to null
						form.setFieldValue('address2', e.target.value || null);
					}}
				/>
				<Group align='center' grow>
					<TextInputInfo
						required
						disabled={!isOwner}
						label='City'
						{...form.getInputProps('city')}
					/>
					<StateInput
						required
						disabled={!isOwner}
						label='State'
						searchable
						{...form.getInputProps('state')}
					/>
				</Group>
				<TextInputInfo
					required
					disabled={!isOwner}
					label='Zip Code'
					{...form.getInputProps('zipCode')}
				/>
			</Paper>
			{isOwner && (
				<Button disabled={submitting} loading={submitting} type='submit' mt='md'>
					Update Account
				</Button>
			)}
		</Box>
	);
};

export default AccountForm;
