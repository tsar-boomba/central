import React from 'react';
import { createStyles, Switch, useMantineColorScheme, Stack } from '@mantine/core';
import { CgSun, CgMoon } from 'react-icons/cg';

const useStyles = createStyles((theme) => ({
	root: {
		position: 'relative',
		'& *': {
			cursor: 'pointer',
		},
	},

	icon: {
		pointerEvents: 'none',
		position: 'absolute',
		zIndex: 1,
		top: 3,
	},

	iconLight: {
		left: 4,
		top: 4,
		color: theme.white,
	},

	iconDark: {
		right: 4,
		color: theme.colors.gray[6],
	},
}));

const ThemeToggle = () => {
	const { colorScheme, toggleColorScheme } = useMantineColorScheme();
	const { classes, cx } = useStyles();

	return (
		<Stack align='center' spacing={0} mx={0}>
			<div className={classes.root}>
				<CgSun className={cx(classes.icon, classes.iconLight)} size={16} />
				<CgMoon className={cx(classes.icon, classes.iconDark)} size={18} />
				<Switch
					checked={colorScheme === 'dark'}
					onChange={() => toggleColorScheme()}
					size='md'
				/>
			</div>
		</Stack>
	);
};

export default ThemeToggle;
