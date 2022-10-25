import Layout from '@/components/Layout';
import type { AppContext, AppInitialProps, AppProps } from 'next/app';
import { useState, useMemo, ReactNode, useEffect } from 'react';
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
import { Spotlight } from '@/components/Spotlight';
import { isAuthed } from '../utils/authUtils';
import { User } from '../types/User';
import useSWR from 'swr';
import { api } from '@/utils/apiHelpers';
import fetcher from '@/utils/swrFetcher';

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

const noLayoutPaths: string[] = ['/register'];
const publicPaths: string[] = ['/login', '/register'];

const MyApp: _App<{
	colorScheme: ColorScheme;
	primaryColor: DefaultMantineColor;
	initialUser?: User;
}> = ({
	Component,
	pageProps,
	router,
	initialUser,
	colorScheme: initialColorScheme,
	primaryColor: initialPrimaryColor,
}) => {
	const isNoLayout = noLayoutPaths.includes(router.pathname);
	const isPublic = publicPaths.includes(router.pathname);

	const {
		data: user,
		error,
		mutate,
	} = useSWR(api('verify'), fetcher, { fallbackData: initialUser });

	useEffect(() => {
		if ((!user || error) && !isPublic) {
			mutate(undefined);
			router.push(`/login?from=${router.pathname}`);
		}
	});

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
				<MantineProvider withGlobalStyles withNormalizeCSS theme={theme as any}>
					<ColorProvider primaryColor={primaryColor} setPrimaryColor={setPrimaryColor}>
						<ModalsProvider>
							<NotificationsProvider>
								<Spotlight>
									{!isNoLayout ? (
										<Layout>
											<Component {...pageProps} />
										</Layout>
									) : (
										<Component {...pageProps} />
									)}
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
