import {
	createPolymorphicComponent,
	MantineColor,
	MantineTheme,
	Paper,
	PaperProps,
} from '@mantine/core';
import { ComponentPropsWithoutRef, forwardRef, PropsWithChildren } from 'react';

type GradientReturn = { from?: MantineColor; to?: MantineColor; deg?: number };

interface Props {
	gradient?: (theme: MantineTheme) => GradientReturn;
}

const handleGradient = (theme: MantineTheme, gradient: GradientReturn): string => {
	const primShade = theme.fn.primaryShade();
	const fromColor = gradient.from
		? gradient.from in theme.colors
			? theme.colors[gradient.from][primShade - 3]
			: gradient.from
		: theme.colors[theme.primaryColor][primShade - 3];

	const toColor = gradient.to
		? gradient.to in theme.colors
			? theme.colors[gradient.to][primShade]
			: gradient.to
		: theme.colors[theme.primaryColor][primShade];

	return `linear-gradient(${gradient.deg || -60}deg, ${fromColor} 0%, ${toColor} 100%)`;
};

const _GradientCard = forwardRef<
	HTMLDivElement,
	PropsWithChildren<Props & PaperProps & ComponentPropsWithoutRef<'div'>>
>(({ children, gradient = () => ({}), sx, ...props }, ref) => {
	if (typeof sx === 'function' || Array.isArray(sx))
		throw new Error('sx prop may not be a function or array for this component.');

	return (
		<Paper
			component='div'
			ref={ref}
			{...props}
			sx={(theme) => ({ ...sx, backgroundImage: handleGradient(theme, gradient(theme)) })}
		>
			{children}
		</Paper>
	);
});

const GradientCard = createPolymorphicComponent<
	HTMLDivElement,
	PropsWithChildren<Props & PaperProps & ComponentPropsWithoutRef<'div'>>
>(_GradientCard);

export default GradientCard;
