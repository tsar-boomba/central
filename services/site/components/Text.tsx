import { Box, BoxProps } from '@mantine/core';

type TextElms = {
	h1?: boolean;
	h2?: boolean;
	h3?: boolean;
	h4?: boolean;
	h5?: boolean;
	h6?: boolean;
	p?: boolean;
};

const getComponent = ({ h1, h2, h3, h4, h5, h6, p }: TextElms) => {
	if (h1) return 'h1';
	if (h2) return 'h2';
	if (h3) return 'h3';
	if (h4) return 'h4';
	if (h5) return 'h5';
	if (h6) return 'h6';
	if (p) return 'p';
	return undefined;
};

const Text = <C extends keyof JSX.IntrinsicElements>(props: BoxProps<C> & TextElms) => {
	return (
		<Box<C>
			component={getComponent(props)}
			sx={(theme) => theme.other.defaultTextColor(theme)}
			{...props}
		>
			{props.children}
		</Box>
	);
};

export default Text;
