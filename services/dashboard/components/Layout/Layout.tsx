import { PropsWithChildren } from 'react';
import { AppShell, ScrollArea, Stack } from '@mantine/core';
import Navbar from './Navbar';
import { linkGroups } from './linkGroups';
import Header from './Header';
import { useLayoutStyles } from './LayoutStyles';
import { useDisclosure, useHotkeys } from '@mantine/hooks';

const Layout: React.FC<PropsWithChildren<unknown>> = ({ children }) => {
	const [opened, handlers] = useDisclosure(false);
	useHotkeys([['mod+b', handlers.toggle]]);
	const { classes } = useLayoutStyles();

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
