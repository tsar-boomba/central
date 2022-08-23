import { Center, Group, Text, useMantineTheme } from '@mantine/core';

const ServerError = () => {
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
					500
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
					Internal Server Error
				</Text>
			</Group>
		</Center>
	);
};

export default ServerError;
