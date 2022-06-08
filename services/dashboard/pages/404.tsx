import { Center, Group, Text, useMantineTheme } from '@mantine/core';

const NotFound = () => {
	const theme = useMantineTheme();

	return (
		<Center sx={{ width: '100%', height: '100%' }}>
			<Group spacing={16}>
				<Text
					component='h1'
					p={14}
					sx={{ fontSize: 72, fontWeight: 100, borderRight: '2px solid' }}
					color='dimmed'
				>
					404
				</Text>
				<Text
					component='h1'
					variant='gradient'
					gradient={{
						from: theme.colors[theme.primaryColor][8],
						to: theme.colors[theme.primaryColor][6],
						deg: 75,
					}}
					sx={{ fontSize: 62 }}
				>
					Not Found
				</Text>
			</Group>
		</Center>
	);
};

export const getStaticProps = () => ({ props: {} });

export default NotFound;
