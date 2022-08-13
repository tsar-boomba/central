import Layout from '@/components/Layout';
import type { AppContext, AppInitialProps, AppProps } from 'next/app';
import { useState, useMemo, ReactNode } from 'react';
import Head from 'next/head';
import {
	ColorScheme,
	ColorSchemeProvider,
	DefaultMantineColor,
	DEFAULT_THEME,
	MantineProvider,
	MantineThemeOverride,
} from '@mantine/core';
import { ModalsProvider } from '@mantine/modals';
import { getCookie, parse, setCookie } from 'ez-cookies';
import App from 'next/app';
import { NotificationsProvider } from '@/components/NotificationsProvider';
import { ColorProvider } from '@/components/ColorProvider';
import { UserProvider } from '@/components/UserProvider';
import { Spotlight } from '@/components/Spotlight';
import { isAuthed } from '../utils/authUtils';
import { User } from '../types/User';

interface _App<P = {}> {
	(props: AppProps & P): ReactNode;
	getInitialProps(ctx: AppContext): Promise<P & AppInitialProps>;
}

const getTheme = (
	colorScheme: ColorScheme,
	primaryColor: DefaultMantineColor,
): MantineThemeOverride => ({
	...DEFAULT_THEME,
	colorScheme,
	primaryColor,
	cursorType: 'pointer',
	breakpoints: {
		...DEFAULT_THEME.breakpoints,
		xxl: 1550,
	},
});

const noLayoutPaths: string[] = [];

const MyApp: _App<{
	colorScheme: ColorScheme;
	primaryColor: DefaultMantineColor;
	user?: User;
}> = ({
	Component,
	pageProps,
	router,
	user,
	colorScheme: initialColorScheme,
	primaryColor: initialPrimaryColor,
}) => {
	const isNoLayout = noLayoutPaths.includes(router.pathname);

	const [primaryColor, _setPrimaryColor] = useState(initialPrimaryColor);
	const [colorScheme, setColorScheme] = useState(initialColorScheme);

	const toggleColorScheme = (value?: 'dark' | 'light') => {
		const nextColorScheme = colorScheme === 'dark' ? 'light' : 'dark';
		setColorScheme(value || nextColorScheme);
		setCookie('colorScheme', value || nextColorScheme, { maxAge: 60 * 60 * 24 * 365 });
	};

	const setPrimaryColor = (color: DefaultMantineColor) => {
		_setPrimaryColor(color);
		setCookie('primaryColor', color, { maxAge: 60 * 60 * 24 * 365 });
	};

	const theme = useMemo(() => getTheme(colorScheme, primaryColor), [colorScheme, primaryColor]);

	return (
		<>
			<Head>
				<meta charSet='UTF-8' />
				<link
					rel='icon'
					href='data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 256 256%22><text y=%22203%22 font-size=%22224%22>🚚</text></svg>'
				/>
				<meta
					name='viewport'
					content='minimum-scale=1, initial-scale=1, width=device-width'
				/>
				<title>Dashboard | Milky Web</title>
			</Head>
			<ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
				<MantineProvider withGlobalStyles withNormalizeCSS theme={theme}>
					<ColorProvider primaryColor={primaryColor} setPrimaryColor={setPrimaryColor}>
						<ModalsProvider>
							<NotificationsProvider>
								<Spotlight>
									<UserProvider fallback={user}>
										{!isNoLayout ? (
											<Layout>
												<Component {...pageProps} />
											</Layout>
										) : (
											<Component {...pageProps} />
										)}
									</UserProvider>
								</Spotlight>
							</NotificationsProvider>
						</ModalsProvider>
					</ColorProvider>
				</MantineProvider>
			</ColorSchemeProvider>
		</>
	);
};

MyApp.getInitialProps = async (appCtx) => {
	const appProps = await App.getInitialProps(appCtx);
	const cookies = parse(appCtx.ctx.req?.headers.cookie || '');

	return {
		...appProps,
		user: await isAuthed({ req: { cookies } }),
		colorScheme: (getCookie('colorScheme', { req: appCtx.ctx.req }) || 'light') as ColorScheme,
		primaryColor: getCookie('primaryColor', { req: appCtx.ctx.req }) || 'orange',
	};
};

export default MyApp;
