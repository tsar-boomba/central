import { CgDatabase, CgHome, CgUser } from 'react-icons/cg';
import { LinksGroupProps } from './NavbarLinksGroup';

export const linkGroups: LinksGroupProps[] = [
	{
		icon: CgHome,
		label: 'Home',
		link: '/',
	},
	{
		icon: CgDatabase,
		label: 'Instances',
		link: '/instances',
	},
	{
		icon: CgUser,
		label: 'Users',
		link: '/users',
	},
];
