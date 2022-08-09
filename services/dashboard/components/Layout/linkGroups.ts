import { IconDatabase, IconHome, IconSettings, IconUser } from '@tabler/icons';
import { LinksGroupProps } from './NavbarLinksGroup';

export const linkGroups: LinksGroupProps[] = [
	{
		icon: IconHome,
		label: 'Home',
		link: '/',
	},
	{
		icon: IconDatabase,
		label: 'Instances',
		link: '/instances',
	},
	{
		icon: IconUser,
		label: 'Users',
		link: '/users',
	},
	{
		icon: IconSettings,
		label: 'Account',
		link: '/account',
	},
];
