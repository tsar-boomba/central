import { Center, Loader } from '@mantine/core';
import dynamic from 'next/dynamic';

const RichTextEditor = dynamic(() => import('@mantine/rte'), {
	ssr: false,
	loading: () => (
		<Center>
			<Loader size='xl' />
		</Center>
	),
});

export default RichTextEditor;
