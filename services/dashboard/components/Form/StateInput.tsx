import { Select, SelectProps } from '@mantine/core';
import { forwardRef, useMemo } from 'react';
import { stateMap } from '../../utils/states';

type Props = React.ComponentPropsWithoutRef<'input'> & SelectProps;

const StateInput = forwardRef<HTMLInputElement, Props>(({ data: _data, ...props }, ref) => {
	const data = useMemo(() => {
		const selectData: { label: string; value: string }[] = [];
		for (const [abbr, name] of stateMap) {
			selectData.push({ label: `${name} (${abbr})`, value: abbr });
		}
		return selectData;
	}, []);
	return <Select data={data} {...props} ref={ref} />;
});

export default StateInput;
