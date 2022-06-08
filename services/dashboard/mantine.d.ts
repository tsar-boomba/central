import { MantineTheme } from '@mantine/core';

export declare module '@mantine/core' {
	export declare type MantineSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl' | 'xxl';

	export interface MantineThemeOverride extends MantineTheme {
		breakpoints: {
			xxl: number;
		};
	}

	export interface MantineThemeOther {
		flexCenter: (column?: boolean) => {
			display: 'flex';
			alignItems: 'center';
			justifyContent: 'center';
			flexDirection?: 'column' | 'row';
		};
		defaultTextColor: (theme: MantineTheme) => { color: 'white' | 'black' };
	}
}
