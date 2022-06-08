import {
	createStyles,
	Header as MantineHeader,
	Group,
	Burger,
	Text,
	useMantineTheme,
	MediaQuery,
} from '@mantine/core';
import Link from 'next/link';

const useStyles = createStyles((theme) => ({
	header: {
		paddingLeft: theme.spacing.md,
		paddingRight: theme.spacing.md,
	},

	inner: {
		height: 56,
		display: 'flex',
		justifyContent: 'space-between',
		alignItems: 'center',
	},

	links: {
		[theme.fn.smallerThan('md')]: {
			display: 'none',
		},
	},

	search: {
		[theme.fn.smallerThan('xs')]: {
			display: 'none',
		},
	},

	link: {
		display: 'block',
		lineHeight: 1,
		padding: '8px 12px',
		borderRadius: theme.radius.sm,
		textDecoration: 'none',
		color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
		fontSize: theme.fontSizes.sm,
		fontWeight: 500,

		'&:hover': {
			backgroundColor:
				theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
		},
	},
}));

interface HeaderProps {
	navState: [boolean, (value?: boolean) => void];
}

const Header: React.VFC<HeaderProps> = ({ navState }) => {
	const [opened, toggleOpened] = navState;
	const theme = useMantineTheme();
	const { classes } = useStyles();

	return (
		<MantineHeader height={56} className={classes.header} mb={0}>
			<div className={classes.inner}>
				<Group>
					<MediaQuery largerThan='sm' styles={{ display: 'none' }}>
						<Burger opened={opened} onClick={() => toggleOpened()} size='sm' />
					</MediaQuery>
					<Link href='/' passHref>
						<Text
							component='a'
							variant='gradient'
							gradient={{
								from: theme.colors[theme.primaryColor][8],
								to: theme.colors[theme.primaryColor][6],
								deg: 75,
							}}
							sx={{ fontSize: 18, fontWeight: 900 }}
						>
							PUDO
						</Text>
					</Link>
				</Group>
			</div>
		</MantineHeader>
	);
};

export default Header;
