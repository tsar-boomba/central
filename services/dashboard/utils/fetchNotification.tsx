import { NotificationProps, showNotification, updateNotification } from '@mantine/notifications';
import { IconCheck, IconX } from '@tabler/icons';

export const fetchNotification = (
	id: string,
	options: NotificationProps = { message: 'Fetching...' },
) => {
	showNotification({ id, loading: true, ...options });

	const success = (options: NotificationProps = { message: 'Successful fetch 😀.' }) =>
		updateNotification({ id, icon: <IconCheck size={18} />, color: 'green', ...options });
	const fail = (options: NotificationProps = { message: 'There was an error 😔.' }) =>
		updateNotification({ id, icon: <IconX size={18} />, color: 'red', ...options });

	return [success, fail];
};
