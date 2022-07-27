import React, { useState } from 'react';
import { Group, Box, Collapse, ThemeIcon, Text, UnstyledButton, createStyles } from '@mantine/core';
import { CgChevronLeft, CgChevronRight } from 'react-icons/cg';
import Link from 'next/link';
import { LinkOrFragment } from '../LinkOrFragment';
import { useRouter } from 'next/router';

const useStyles = createStyles((theme) => ({
	control: {
		fontWeight: 500,
		display: 'block',
		width: '100%',
		padding: `${theme.spacing.xs}px ${theme.spacing.md}px`,
		color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.black,
		fontSize: theme.fontSizes.sm,

		'&:hover': {
			backgroundColor:
				theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
			color: theme.colorScheme === 'dark' ? theme.white : theme.black,
		},
	},

	link: {
		fontWeight: 500,
		display: 'block',
		textDecoration: 'none',
		padding: `${theme.spacing.xs}px ${theme.spacing.md}px`,
		paddingLeft: 31,
		marginLeft: 30,
		fontSize: theme.fontSizes.sm,
		color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
		borderLeft: `1px solid ${
			theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[3]
		}`,

		'&:hover': {
			backgroundColor:
				theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
			color: theme.colorScheme === 'dark' ? theme.white : theme.black,
		},
	},

	active: {
		backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[1],
		color: theme.colorScheme === 'dark' ? theme.white : theme.black,
	},

	chevron: {
		transition: 'transform 200ms ease',
	},
}));

export interface LinksGroupProps {
	icon: React.VFC<{ size: number }>;
	label: string;
	link?: string;
	initiallyOpened?: boolean;
	links?: { label: string; link: string }[];
}

interface ComponentProps {
	handlers: { open: () => void; close: () => void; toggle: () => void };
}

const LinksGroup: React.VFC<LinksGroupProps & ComponentProps> = ({
	icon: Icon,
	label,
	initiallyOpened,
	link,
	links,
	handlers: navHandlers,
}) => {
	const router = useRouter();
	const { classes, cx, theme } = useStyles();
	const hasLinks = Array.isArray(links);
	const [opened, setOpened] = useState(initiallyOpened || false);
	const ChevronIcon = theme.dir === 'ltr' ? CgChevronRight : CgChevronLeft;
	let childActive = false;
	const items = (hasLinks ? links : []).map((link) => {
		const linkActive = router.asPath === link.link;
		if (linkActive) childActive = true;
		return (
			<Link href={link.link} passHref key={link.label}>
				<Text<'a'>
					component='a'
					className={cx(classes.link, {
						[classes.active]: linkActive,
					})}
					onClick={navHandlers.close}
				>
					{link.label}
				</Text>
			</Link>
		);
	});

	return (
		<>
			<LinkOrFragment href={link} cond={!hasLinks}>
				<UnstyledButton
					component={hasLinks ? 'button' : 'a'}
					onClick={hasLinks ? () => setOpened((o) => !o) : navHandlers.close}
					className={cx(classes.control, {
						[classes.active]: childActive || router.asPath === link,
					})}
				>
					<Group position='apart' spacing={0}>
						<Box sx={{ display: 'flex', alignItems: 'center' }}>
							<ThemeIcon variant='light' size={30}>
								<Icon size={18} />
							</ThemeIcon>
							<Box ml='md'>{label}</Box>
						</Box>
						{hasLinks && (
							<ChevronIcon
								className={classes.chevron}
								size={14}
								style={{
									transform: opened
										? `rotate(${theme.dir === 'rtl' ? -90 : 90}deg)`
										: 'none',
								}}
							/>
						)}
					</Group>
				</UnstyledButton>
			</LinkOrFragment>
			{hasLinks ? <Collapse in={opened}>{items}</Collapse> : <></>}
		</>
	);
};

export default LinksGroup;
