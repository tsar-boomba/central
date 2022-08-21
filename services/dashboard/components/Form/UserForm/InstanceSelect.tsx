import { useUser } from '@/components/UserProvider';
import { Instance } from '@/types/Instance';
import { api } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';
import { Group, Loader, MultiSelect, MultiSelectProps, Text } from '@mantine/core';
import { forwardRef } from 'react';
import useSWR from 'swr';

type Props = Omit<
	MultiSelectProps &
		React.ComponentPropsWithoutRef<'select'> & {
			create?: boolean;
		},
	'data'
>;
type Data = { value: string; label: string; instance: Instance };
type ItemProps = Data & React.ComponentPropsWithoutRef<'div'>;

const Item = forwardRef<HTMLDivElement, ItemProps>(({ label, ...props }, ref) => (
	<div ref={ref} {...props}>
		<Group noWrap>
			<div>
				<Text weight={600} size='sm'>
					{label}
				</Text>
			</div>
		</Group>
	</div>
));

const InstanceSelect = forwardRef<HTMLInputElement, Props>(({ create, ...props }, ref) => {
	const { user } = useUser();
	const { data: instances } = useSWR<Instance[]>(
		user ? api(`accounts/${user.accountId}/instances`) : null,
		fetcher,
	);
	if (!user)
		return (
			<Group>
				<Loader />
			</Group>
		);
	return (
		<MultiSelect
			ref={ref}
			data={
				instances
					? instances.map((instance) => ({ value: instance.id, label: instance.name }))
					: []
			}
			disabled={!instances}
			rightSection={instances ? undefined : <Loader size='xs' />}
			itemComponent={Item}
			{...props}
		/>
	);
});

export default InstanceSelect;
