import { PropsWithChildren } from 'react';
import Header from './Header';
import { useLayoutStyles } from './LayoutStyles';

const Layout: React.FC<PropsWithChildren<unknown>> = ({ children }) => {
	const { classes } = useLayoutStyles();

	return (
		<div className={classes.page}>
			<Header links={[{ label: 'Home', link: '/' }]} />
			<main className={classes.main}>{children}</main>
		</div>
	);
};

export default Layout;
