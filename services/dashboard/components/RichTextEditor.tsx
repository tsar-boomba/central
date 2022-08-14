import { Loader } from '@mantine/core';
import dynamic from 'next/dynamic';

const RichTextEditor = dynamic(() => import('@mantine/rte'), {
	ssr: false,
	loading: () => <Loader size='xl' />,
});

export default RichTextEditor;
