import { useAccount } from '@/components/UserProvider';
import { Resource, Role } from '@/types/User';
import { NewUser } from '@/types/utils';
import { callApi } from '@/utils/apiHelpers';
import { fetchNotification } from '@/utils/fetchNotification';
import {
	Box,
	Button,
	Group,
	MultiSelect,
	Paper,
	PasswordInput,
	Progress,
	Text,
	Textarea,
	TextInput,
	Transition,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { FormRulesRecord } from '@mantine/form/lib/types';
import { openConfirmModal } from '@mantine/modals';
import { useRouter } from 'next/router';
import { useState } from 'react';
import InstanceSelect from './InstanceSelect';
import { getStrength, PasswordRequirement, requirements } from './Password';
import RoleSelect from './RoleSelect';

interface Props {
	defaultUser?: NewUser & { confirmPass?: string };
	create?: boolean;
	id?: number;
}

type FormValues = NewUser & { confirmPass?: string };

const userValidation: (create: boolean | undefined) => FormRulesRecord<FormValues> = (create) => ({
	username: (v) => (v.length > 0 ? null : 'Username cannot be empty.'),
	firstName: (v) => (v.length > 0 ? null : 'First Name cannot be empty.'),
	lastName: (v) => (v.length > 0 ? null : 'Last Name cannot be empty.'),
	password: (v) =>
		// if updating with no length dont check strength
		(v.length <= 0 && !create) || getStrength(v) >= 100
			? null
			: 'Password must meet requirements.',
	confirmPass: (v: string | undefined, values: any) =>
		v === values.password ? null : 'Passwords must match.',
	active: (v: boolean) => (typeof v === 'boolean' ? null : 'Active must be a boolean'),
	createPerms: (v) =>
		v.every((r) => r === Resource.Carrier || r === Resource.Load || r === Resource.Shipper)
			? null
			: 'Invalid resource included.',
	deletePerms: (v) =>
		v.every((r) => r === Resource.Carrier || r === Resource.Load || r === Resource.Shipper)
			? null
			: 'Invalid resource included.',
	updatePerms: (v) =>
		v.every((r) => r === Resource.Carrier || r === Resource.Load || r === Resource.Shipper)
			? null
			: 'Invalid resource included.',
});

const createDefaultUser: FormValues = {
	firstName: '',
	lastName: '',
	password: '',
	confirmPass: '',
	username: '',
	createPerms: [],
	deletePerms: [],
	updatePerms: [],
	active: true,
	instances: [],
	role: Role.User,
	notes: '',
	accountId: '', // doesn't matter
};

const permissionsData = [
	{ value: Resource.Load, label: 'Loads' },
	{ value: Resource.Carrier, label: 'Carriers' },
	{ value: Resource.Shipper, label: 'Shippers' },
];

const UserForm = ({ id, defaultUser = createDefaultUser, create }: Props) => {
	const { account } = useAccount();
	const router = useRouter();
	const [submitting, setSubmitting] = useState(false);
	const form = useForm<FormValues>({
		// spread so that when updating, password fields will be empty
		initialValues: { ...createDefaultUser, ...defaultUser },
		validate: userValidation(create),
	});

	// make sure password is strong
	const checks = requirements.map((requirement, index) => (
		<PasswordRequirement
			key={index}
			label={requirement.label}
			meets={requirement.re.test(form.values.password)}
		/>
	));
	const strength = getStrength(form.values.password);
	const color = strength === 100 ? 'green' : strength > 50 ? 'yellow' : 'red';

	const submit = (data: FormValues) => {
		if (!account) return console.log('No account, cannot submit form.');
		console.log('sent', {
			...data,
			accountId: account.id,
			notes: data.notes === '' ? null : data.notes,
			// basically saying dont update password if its empty
			password: data.password === '' ? (undefined as any) : data.password,
		});
		setSubmitting(true);
		const [ok, err] = fetchNotification('create-user', {
			message: create ? 'Creating user...' : 'Updating user...',
		});
		callApi<NewUser>({
			route: create ? 'users' : `users/${id}`,
			body: {
				...data,
				accountId: account.id,
				notes: data.notes === '' ? null : data.notes,
				// basically saying dont update password if its empty
				password: data.password === '' ? (undefined as any) : data.password,
			},
			method: create ? 'POST' : 'PUT',
		})
			.then((res) => {
				if (res.ok) {
					ok({
						message: create
							? 'Successfully created user. 😁'
							: 'Successfully updated user. 😁',
					});
					create && router.push('/users');
				} else {
					err({
						message: create ? 'Failed to create user. 😔' : 'Failed to update user. 😔',
					});
				}
			})
			.finally(() => setSubmitting(false));
	};
	const onSubmit = (data: FormValues) =>
		openConfirmModal({
			title: 'Create User?',
			children: (
				<Text>
					Are you sure you want to create a user. This will increase your usage for the
					month.
				</Text>
			),
			labels: { confirm: "I'm Sure", cancel: 'Go Back' },
			confirmProps: { color: 'green' },
			onConfirm: () => submit(data),
		});

	return (
		<Box
			component='form'
			sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}
			onSubmit={form.onSubmit(create ? onSubmit : submit)}
		>
			<Paper>
				<TextInput required label='Username' {...form.getInputProps('username')} />
				<Group align='center' grow>
					<TextInput required label='First Name' {...form.getInputProps('firstName')} />
					<TextInput required label='Last Name' {...form.getInputProps('lastName')} />
				</Group>
				<Group grow>
					<PasswordInput
						required={create}
						label='Password'
						{...form.getInputProps('password')}
					/>
					<PasswordInput
						required={create}
						label='Confirm Pass'
						{...form.getInputProps('confirmPass')}
					/>
				</Group>

				<Transition
					mounted={create || form.values.password.length > 0}
					transition='scale-y'
				>
					{(style) => (
						<Box style={style} sx={{ width: '100%' }} mt='md'>
							<Progress color={color} value={strength} size={5} mb='md' />
							<PasswordRequirement
								label='Includes at least 6 characters'
								meets={form.values.password.length > 5}
							/>
							{checks}
						</Box>
					)}
				</Transition>
				<Paper mt='md'>
					<MultiSelect
						withinPortal
						label='Update Permissions'
						description='What this user is allowed to update in instances'
						data={permissionsData}
						clearable
						{...form.getInputProps('updatePerms')}
					/>
					<MultiSelect
						withinPortal
						label='Create Permissions'
						description='What this user is allowed to create in instances'
						data={permissionsData}
						clearable
						{...form.getInputProps('createPerms')}
					/>
					<MultiSelect
						withinPortal
						label='Delete Permissions'
						description='What this user is allowed to delete in instances'
						data={permissionsData}
						clearable
						{...form.getInputProps('deletePerms')}
					/>
				</Paper>
				<RoleSelect
					required
					label='Role'
					description='What permissions this user has on Milky Web'
					create={create}
					id={id}
					{...form.getInputProps('role')}
				/>
				<InstanceSelect
					required
					label='Instances'
					description='What instances this user has access to'
					create={create}
					{...form.getInputProps('instances')}
				/>
				<Textarea
					label='Notes'
					autosize
					minRows={2}
					{...form.getInputProps('notes')}
					onChange={(e) =>
						form.setFieldValue(
							'bottomText',
							e.target.value !== '' ? e.target.value : null,
						)
					}
				/>
			</Paper>
			<Button disabled={submitting} loading={submitting} type='submit' mt='md'>
				{create ? 'Create User' : 'Update User'}
			</Button>
		</Box>
	);
};

export default UserForm;
