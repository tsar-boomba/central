import { createStyles } from '@mantine/core';

export const useLayoutStyles = createStyles(() => ({
	main: {
		display: 'flex',
		flexDirection: 'column',
		alignItems: 'center',
		alignSelf: 'center',
		padding: '56px 0 0 0',
		position: 'relative',
		bottom: '100%',
		flexGrow: 1,
		width: '100%',
	},

	page: {
		display: 'flex',
		flexDirection: 'column',
		minHeight: '100vh',
		transition: 'background-color 0.3s ease',
	},
}));
