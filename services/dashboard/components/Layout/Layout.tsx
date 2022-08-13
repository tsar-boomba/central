import { PropsWithChildren, useMemo } from 'react';
import { AppShell, ScrollArea, Stack } from '@mantine/core';
import Navbar from './Navbar';
import Header from './Header';
import { useLayoutStyles } from './LayoutStyles';
import { useDisclosure, useHotkeys } from '@mantine/hooks';
import { LinksGroupProps } from './NavbarLinksGroup';
import { IconDatabase, IconHome, IconSettings, IconUser } from '@tabler/icons';
import { useUser } from '../UserProvider';
import { Role } from '@/types/User';

const Layout: React.FC<PropsWithChildren<unknown>> = ({ children }) => {
	const [opened, handlers] = useDisclosure(false);
	useHotkeys([['mod+b', handlers.toggle]]);
	const { classes } = useLayoutStyles();
	const { user } = useUser();

	const linkGroups: LinksGroupProps[] = useMemo(() => {
		const links = [
			{
				icon: IconHome,
				label: 'Home',
				link: '/',
			},
			{
				icon: IconDatabase,
				label: 'Instances',
				link: '/instances',
			},
			{
				icon: IconUser,
				label: 'Users',
				link: '/users',
			},
		];

		// only show certain links if have a certain role
		if (user?.role === Role.Owner)
			links.push({
				icon: IconSettings,
				label: 'Account',
				link: '/account',
			});

		return links;
	}, [user]);

	return (
		<div className={classes.page}>
			<AppShell
				padding={0}
				navbarOffsetBreakpoint='sm'
				fixed
				header={<Header navState={[opened, handlers]} />}
				navbar={<Navbar navState={[opened, handlers]} linkGroups={linkGroups} />}
			>
				<ScrollArea
					p='md'
					className={classes.main}
					// styles={{ viewport: { '& > div': { height: '100%' } } }}
				>
					<Stack align='center' spacing={0}>
						{children}
					</Stack>
				</ScrollArea>
			</AppShell>
		</div>
	);
};

export default Layout;
