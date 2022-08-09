import React, { useState } from 'react';
import { ThemeIcon, NavLink } from '@mantine/core';
import Link from 'next/link';
import { LinkOrFragment } from '../LinkOrFragment';
import { useRouter } from 'next/router';
import { TablerIcon } from '@tabler/icons';

export interface LinksGroupProps {
	icon: TablerIcon;
	label: string;
	link?: string;
	initiallyOpened?: boolean;
	links?: { label: string; link: string }[];
}

interface ComponentProps {
	handlers: { open: () => void; close: () => void; toggle: () => void };
}

const LinksGroup = ({
	icon: Icon,
	label,
	link,
	links,
	handlers: navHandlers,
}: LinksGroupProps & ComponentProps) => {
	const router = useRouter();
	const hasLinks = Array.isArray(links);
	const [childActive, setChildActive] = useState(false);
	const items = (hasLinks ? links : []).map((link) => {
		const linkActive = router.asPath === link.link;
		if (linkActive) setChildActive(true);
		return (
			<Link href={link.link} passHref key={link.label}>
				<NavLink component='a' active={linkActive} onClick={navHandlers.close}>
					{link.label}
				</NavLink>
			</Link>
		);
	});

	return (
		<LinkOrFragment href={link} cond={!hasLinks}>
			<NavLink
				py='sm'
				component={hasLinks ? 'button' : 'a'}
				icon={
					<ThemeIcon variant='light' size={30}>
						<Icon size={18} />
					</ThemeIcon>
				}
				label={label}
				active={link ? router.asPath === link : childActive}
			>
				{hasLinks && items}
			</NavLink>
		</LinkOrFragment>
	);
};

export default LinksGroup;
