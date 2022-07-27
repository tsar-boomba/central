import { Badge, Center, createStyles, Group, UnstyledButton, Text } from '@mantine/core';
import { SpotlightAction, SpotlightActionProps, SpotlightProvider } from '@mantine/spotlight';
import { useRouter } from 'next/router';
import { PropsWithChildren, useMemo } from 'react';
import { CgDatabase, CgHome, CgUser } from 'react-icons/cg';

const useStyles = createStyles((theme) => ({
	action: {
		position: 'relative',
		display: 'block',
		width: '100%',
		padding: '10px 12px',
		borderRadius: theme.radius.sm,
	},

	actionHovered: {
		backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[1],
	},

	actionBody: {
		flex: 1,
	},
}));

const CustomAction = ({
	action,
	styles,
	classNames,
	hovered,
	onTrigger,
	...others
}: SpotlightActionProps) => {
	const { classes, cx } = useStyles(void 0, {
		styles: styles as any,
		classNames,
		name: 'Spotlight',
	});

	return (
		<UnstyledButton
			className={cx(classes.action, { [classes.actionHovered]: hovered })}
			tabIndex={-1}
			onMouseDown={(e: { preventDefault: () => void }) => e.preventDefault()}
			onClick={onTrigger}
			{...others}
		>
			<Group noWrap>
				{action.icon && <Center>{action.icon}</Center>}

				<div className={classes.actionBody}>
					<Text>{action.title}</Text>

					{action.description && (
						<Text color='dimmed' size='xs'>
							{action.description}
						</Text>
					)}
				</div>

				{action.new && <Badge>new</Badge>}
			</Group>
		</UnstyledButton>
	);
};

const Spotlight: React.FC<PropsWithChildren<unknown>> = ({ children }) => {
	const router = useRouter();

	const actions = useMemo<SpotlightAction[]>(
		() => [
			{
				title: 'Home',
				icon: <CgHome size={24} />,
				onTrigger: () => router.push('/'),
				href: '/',
			},
			{
				title: 'Instances',
				icon: <CgDatabase size={24} />,
				onTrigger: () => router.push('/load-status'),
				href: 'load-status',
			},
			{
				title: 'Users',
				icon: <CgUser size={24} />,
				onTrigger: () => router.push('/records/loads'),
				href: '/records/loads',
			},
		],
		[],
	);

	return (
		<SpotlightProvider
			actions={actions}
			actionComponent={CustomAction}
			shortcut={['mod + k', '/']}
			onSpotlightOpen={() => actions.forEach((action) => router.prefetch(action.href))}
		>
			{children}
		</SpotlightProvider>
	);
};

export default Spotlight;
