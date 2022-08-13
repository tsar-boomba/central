import { DEFAULT_SSR } from '@/utils/authUtils';
import {
	Container,
	createStyles,
	Group,
	Stack,
	Text,
	ThemeIcon,
	Transition,
	UnstyledButton,
} from '@mantine/core';
import { IconBolt, IconDatabase, IconUser } from '@tabler/icons';
import Link from 'next/link';
import GradientCard from '../components/GradientCard';
import { useSubStatus } from '../utils/useSubStatus';

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

		subButton: {
			display: 'flex',
			alignItems: 'center',
			color: colors.color,
			backgroundColor: colors.background,
			padding: theme.spacing.xl,
			flex: 1,
			borderRadius: theme.radius.sm,
			...theme.fn.focusStyles(),
		},
	};
});

const Home = () => {
	const { classes, theme } = useStyles();
	const { status } = useSubStatus();

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
					Welcome to the Milky Web Dashboard
				</Text>
			</Container>
			<Container size='sm' mt={64}>
				<Stack align='center'>
					<Group position='center' align='stretch'>
						<Link href='/instances' passHref>
							<UnstyledButton component='a' className={classes.control}>
								<ThemeIcon radius='xl' size={64} variant='light'>
									<IconDatabase size={32} />
								</ThemeIcon>
								<Stack spacing={0} ml='md'>
									<Text className={classes.controlTitle}>Instances</Text>
									<Text color='dimmed'>
										Manage your instances and their settings
									</Text>
								</Stack>
							</UnstyledButton>
						</Link>
						<Link href='/users' passHref>
							<UnstyledButton component='a' className={classes.control}>
								<ThemeIcon radius='xl' size={64} variant='light'>
									<IconUser size={32} />
								</ThemeIcon>
								<Stack spacing={0} ml='md'>
									<Text className={classes.controlTitle}>Users</Text>
									<Text color='dimmed'>
										Manage your users and their permissions
									</Text>
								</Stack>
							</UnstyledButton>
						</Link>
					</Group>
					<Transition mounted={status === undefined} transition='fade' duration={500}>
						{(styles) => (
							<Link href='/subscribe' passHref>
								<GradientCard
									sx={{ color: 'white', maxWidth: 540 }}
									style={styles}
									className={classes.subButton}
									component='a'
								>
									<ThemeIcon radius='xl' size={64} color='white'>
										<IconBolt
											color={
												theme.colors[theme.primaryColor][
													theme.fn.primaryShade()
												]
											}
											size={32}
										/>
									</ThemeIcon>
									<Stack spacing={0} ml='md'>
										<Text className={classes.controlTitle}>Subscribe</Text>
										<Text color='white'>
											Add a payment method and begin managing your loads and
											customers more efficiently.
										</Text>
									</Stack>
								</GradientCard>
							</Link>
						)}
					</Transition>
				</Stack>
			</Container>
		</>
	);
};

export const getServerSideProps = DEFAULT_SSR('/');

export default Home;
