import { DEFAULT_SSR } from '@/utils/authUtils';
import {
	Container,
	createStyles,
	Group,
	Stack,
	Text,
	ThemeIcon,
	UnstyledButton,
} from '@mantine/core';
import Link from 'next/link';
import { CgDatabase, CgUser } from 'react-icons/cg';

const useStyles = createStyles((theme) => {
	const colors = theme.fn.variant({ variant: 'outline' });
	return {
		control: {
			display: 'flex',
			alignItems: 'center',
			color: colors.color,
			backgroundColor: colors.background,
			border: `2px solid ${colors.border}`,
			padding: theme.spacing.xl,
			flex: 1,
			borderRadius: theme.radius.sm,

			'&:hover': {
				backgroundColor: colors.hover,
			},
		},

		controlTitle: {
			fontWeight: 600,
			fontSize: 20,
		},
	};
});

const Home = () => {
	const { classes } = useStyles();

	return (
		<>
			<Container size='sm'>
				<Text
					align='center'
					sx={(theme) => ({
						fontSize: 48,
						fontWeight: 700,
						[theme.fn.smallerThan('md')]: { fontSize: 36 },
					})}
				>
					Welcome to the *name here* Dashboard
				</Text>
			</Container>
			<Container size='sm' mt={64}>
				<Group position='center' align='stretch'>
					<Link href='/instances' passHref>
						<UnstyledButton component='a' className={classes.control}>
							<ThemeIcon radius='xl' size={64} variant='light'>
								<CgDatabase size={32} />
							</ThemeIcon>
							<Stack spacing={0} ml='md'>
								<Text className={classes.controlTitle}>Instances</Text>
								<Text color='dimmed'>Manage your instances and their settings</Text>
							</Stack>
						</UnstyledButton>
					</Link>
					<Link href='/users' passHref>
						<UnstyledButton component='a' className={classes.control}>
							<ThemeIcon radius='xl' size={64} variant='light'>
								<CgUser size={32} />
							</ThemeIcon>
							<Stack spacing={0} ml='md'>
								<Text className={classes.controlTitle}>Users</Text>
								<Text color='dimmed'>Manage your users and their information</Text>
							</Stack>
						</UnstyledButton>
					</Link>
				</Group>
			</Container>
		</>
	);
};

export const getServerSideProps = DEFAULT_SSR('/');

export default Home;
