import { Box, Text } from '@mantine/core';
import { IconCheck, IconX } from '@tabler/icons';

export const requirements = [
	{ re: /[0-9]/, label: 'Includes number' },
	{ re: /[a-z]/, label: 'Includes lowercase letter' },
	{ re: /[A-Z]/, label: 'Includes uppercase letter' },
	{ re: /[$&+,:;=?@#|'<>.^*()%!_-]/, label: 'Includes special symbol' },
];

export const getStrength = (password: string) => {
	let multiplier = password.length > 5 ? 0 : 1;

	requirements.forEach((requirement) => {
		if (!requirement.re.test(password)) {
			multiplier += 1;
		}
	});

	return Math.max(100 - (100 / (requirements.length + 1)) * multiplier, 10);
};

export const PasswordRequirement = ({ meets, label }: { meets: boolean; label: string }) => {
	return (
		<Text
			color={meets ? 'green' : 'red'}
			sx={{ display: 'flex', alignItems: 'center' }}
			mt={7}
			size='sm'
		>
			{meets ? <IconCheck size={14} /> : <IconX size={14} />}{' '}
			<Box ml={10} sx={{ fontWeight: 500 }}>
				{label}
			</Box>
		</Text>
	);
};
