import { Box, Group, Popover, TextInput, TextInputProps } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { forwardRef, ReactNode } from 'react';
import { CgInfo } from 'react-icons/cg';

type Props = React.ComponentPropsWithoutRef<'input'> & TextInputProps & { info?: ReactNode };

const Label = ({ label, info }: { label?: ReactNode; info: ReactNode }) => {
	const [opened, { close, open }] = useDisclosure(false);

	return (
		<Group sx={{ display: 'inline-flex' }}>
			<Popover position='top' withArrow opened={opened}>
				<Popover.Target>
					<Group
						spacing={2}
						align='center'
						onClick={open}
						onTouchStart={open}
						onMouseEnter={open}
						onMouseLeave={close}
					>
						{label}
						<CgInfo size={16} />
					</Group>
				</Popover.Target>
				<Popover.Dropdown sx={{ maxWidth: 200 }}>
					<Box>{info}</Box>
				</Popover.Dropdown>
			</Popover>
		</Group>
	);
};

const TextInputInfo = forwardRef<HTMLInputElement, Props>(({ info, ...props }, ref) => {
	return (
		<TextInput
			ref={ref}
			{...props}
			label={info ? <Label label={props.label} info={info} /> : props.label}
		/>
	);
});

export default TextInputInfo;
