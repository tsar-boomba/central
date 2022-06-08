import Layout from '@/components/Layout';
import type { AppContext, AppInitialProps, AppProps } from 'next/app';
import { useMemo, ReactNode } from 'react';
import Head from 'next/head';
import {
	ColorScheme,
	ColorSchemeProvider,
	DefaultMantineColor,
	DEFAULT_THEME,
	MantineProvider,
	MantineThemeOther,
	MantineThemeOverride,
} from '@mantine/core';
import { ModalsProvider } from '@mantine/modals';
import { NotificationsProvider } from '@/components/NotificationsProvider';
import { ColorProvider } from '@/components/ColorProvider';
import { useColorScheme, useLocalStorage } from '@mantine/hooks';

interface _App<P = {}> {
	(props: AppProps & P): ReactNode;
	getInitialProps?: (ctx: AppContext) => Promise<P & AppInitialProps>;
}

const other: MantineThemeOther = {
	flexCenter: (column) => ({
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center',
		flexDirection: column ? 'column' : 'row',
	}),
	defaultTextColor: (theme) => ({ color: theme.colorScheme === 'dark' ? 'white' : 'black' }),
};

const getTheme = (
	colorScheme: ColorScheme,
	primaryColor: DefaultMantineColor,
): MantineThemeOverride => ({
	...DEFAULT_THEME,
	colorScheme,
	primaryColor,
	other,
	breakpoints: {
		...DEFAULT_THEME.breakpoints,
		xxl: 1550,
	},
});

const MyApp: _App = ({ Component, pageProps }) => {
	const preferredColorScheme = useColorScheme();
	const [colorScheme, setColorScheme] = useLocalStorage({
		key: 'colorScheme',
		defaultValue: preferredColorScheme,
	});
	const [primaryColor, setPrimaryColor] = useLocalStorage<DefaultMantineColor>({
		key: 'primaryColor',
		defaultValue: 'blue',
	});

	const toggleColorScheme = (value?: 'dark' | 'light') => {
		const nextColorScheme = colorScheme === 'dark' ? 'light' : 'dark';
		setColorScheme(value || nextColorScheme);
	};

	const theme = useMemo(() => getTheme(colorScheme, primaryColor), [colorScheme, primaryColor]);

	return (
		<>
			<Head>
				<link rel='icon' href='/icon.png' type='image/png' />
				<meta
					name='viewport'
					content='minimum-scale=1, initial-scale=1, width=device-width'
				/>
				<title>Load Master</title>
			</Head>
			<ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
				<MantineProvider withGlobalStyles withNormalizeCSS theme={theme}>
					<ColorProvider primaryColor={primaryColor} setPrimaryColor={setPrimaryColor}>
						<ModalsProvider>
							<NotificationsProvider>
								<Layout>
									<Component {...pageProps} />
								</Layout>
							</NotificationsProvider>
						</ModalsProvider>
					</ColorProvider>
				</MantineProvider>
			</ColorSchemeProvider>
		</>
	);
};

export default MyApp;
