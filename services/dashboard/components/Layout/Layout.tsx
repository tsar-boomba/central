import { PropsWithChildren } from 'react';
import { AppShell, ScrollArea, Stack } from '@mantine/core';
import Navbar from './Navbar';
import { linkGroups } from './linkGroups';
import Header from './Header';
import { useLayoutStyles } from './LayoutStyles';
import { useBooleanToggle, useHotkeys } from '@mantine/hooks';

const Layout: React.FC<PropsWithChildren<unknown>> = ({ children }) => {
	const [opened, toggleOpened] = useBooleanToggle(false);
	useHotkeys([['mod+b', () => toggleOpened()]]);
	const { classes } = useLayoutStyles();

	return (
		<div className={classes.page}>
			<AppShell
				padding={0}
				navbarOffsetBreakpoint='sm'
				fixed
				header={<Header navState={[opened, toggleOpened]} />}
				navbar={<Navbar navState={[opened, toggleOpened]} linkGroups={linkGroups} />}
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
