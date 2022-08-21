import { useUser } from '@/components/UserProvider';
import { Role } from '@/types/User';
import { higherRole } from '@/utils/authUtils';
import { Group, Loader, Select, SelectProps, Text } from '@mantine/core';
import { forwardRef } from 'react';

type Props = Omit<
	SelectProps &
		React.ComponentPropsWithoutRef<'select'> & { create?: boolean; id: number | undefined },
	'data'
>;
type Data = { value: Role; label: string; description: string };
type ItemProps = Data & React.ComponentPropsWithoutRef<'div'>;

const roleData: Data[] = [
	{ value: Role.User, label: 'User', description: 'A regular user, can only view instances.' },
	{
		value: Role.Moderator,
		label: 'Moderator',
		description:
			'Can create, edit, and delete users, but not instances. Be careful who you give this role.',
	},
	{
		value: Role.Admin,
		label: 'Admin',
		description:
			'All the powers of a Moderator, but can create, edit, and delete instances. Be extra careful who you give this role.',
	},
	{
		value: Role.Owner,
		label: 'Owner',
		description:
			'Owner of the account, has access to all resources, users, and instances. Cannot be set/updated here.',
	},
];

const Item = forwardRef<HTMLDivElement, ItemProps>(({ label, description, ...props }, ref) => (
	<div ref={ref} {...props}>
		<Group noWrap>
			<div>
				<Text weight={600} size='sm'>
					{label}
				</Text>
				<Text size='xs'>{description}</Text>
			</div>
		</Group>
	</div>
));

const RoleSelect = forwardRef<HTMLInputElement, Props>(({ create, id, ...props }, ref) => {
	const { user } = useUser();
	if (!user)
		return (
			<Group>
				<Loader />
			</Group>
		);
	return (
		<Select
			ref={ref}
			// Only show roles which are lower than current user when creating
			// When updating show all roles but disable ones that current user can't set
			data={
				create
					? roleData.filter(({ value }) => higherRole(user?.role, value))
					: roleData.map((data) => ({
							...data,
							disabled: !higherRole(user?.role, data.value),
					  }))
			}
			itemComponent={Item}
			disabled={props.value === Role.Owner || user.id === id}
			{...props}
		/>
	);
});

export default RoleSelect;
