import {
	Navbar as MantineNavbar,
	Group,
	ScrollArea,
	createStyles,
	Text,
	Kbd,
	MediaQuery,
} from '@mantine/core';
import { useOs } from '@mantine/hooks';
import ColorPicker from './ColorPicker';
import LinksGroup, { LinksGroupProps } from './NavbarLinksGroup';
import ThemeToggle from './ThemeSwitch';

const useStyles = createStyles((theme) => ({
	navbar: {
		borderRight: `1px solid ${
			theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[3]
		}`,
		padding: `0px ${theme.spacing.md}px`,
	},

	header: {
		padding: theme.spacing.md,
		paddingTop: 0,
		marginLeft: -theme.spacing.md,
		marginRight: -theme.spacing.md,
		color: theme.colorScheme === 'dark' ? theme.white : theme.black,
		borderBottom: `1px solid ${
			theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[3]
		}`,
	},

	links: {
		marginLeft: -theme.spacing.md,
		marginRight: -theme.spacing.md,
	},

	linksInner: {
		paddingBottom: theme.spacing.xl,
	},

	footer: {
		marginLeft: -theme.spacing.md,
		marginRight: -theme.spacing.md,
		borderTop: `1px solid ${
			theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[3]
		}`,
	},
}));

interface NavbarProps {
	linkGroups: LinksGroupProps[];
	navState: [boolean, { open: () => void; close: () => void; toggle: () => void }];
}

const Navbar: React.VFC<NavbarProps> = ({ linkGroups, navState }) => {
	const { classes } = useStyles();
	const os = useOs();
	const [opened, handlers] = navState;
	const links = linkGroups.map((group) => (
		<LinksGroup {...group} handlers={handlers} key={group.label} />
	));

	return (
		<MantineNavbar
			hidden={!opened}
			hiddenBreakpoint='sm'
			width={{ sm: 200, lg: 300 }}
			px='md'
			className={classes.navbar}
		>
			<MantineNavbar.Section grow className={classes.links} component={ScrollArea}>
				<div className={classes.linksInner}>{links}</div>
			</MantineNavbar.Section>
			<MediaQuery smallerThan='sm' styles={{ display: 'none' }}>
				<MantineNavbar.Section>
					<Text color='dimmed'>
						Tip: You can use{' '}
						<span>
							"<Kbd>{os !== 'macos' ? 'CTRL' : '⌘'}</Kbd> + <Kbd>K</Kbd>" or "
							<Kbd>/</Kbd>"
						</span>{' '}
						to open the spotlight for fast navigation
					</Text>
				</MantineNavbar.Section>
			</MediaQuery>

			<MantineNavbar.Section>
				<Group pb='md' position='center'>
					<ThemeToggle /> <ColorPicker />
				</Group>
			</MantineNavbar.Section>

			<MantineNavbar.Section className={classes.footer} pb='sm'>
				© {new Date().getFullYear()} Isaiah Gamble
			</MantineNavbar.Section>
		</MantineNavbar>
	);
};

export default Navbar;
