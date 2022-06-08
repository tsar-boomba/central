import { NotificationProps, showNotification, updateNotification } from '@mantine/notifications';
import { CgCheck, CgClose } from 'react-icons/cg';

export const fetchNotification = (
	id: string,
	options: NotificationProps = { message: 'Fetching...' },
) => {
	showNotification({ id, loading: true, ...options });

	const success = (options: NotificationProps = { message: 'Successful fetch 😀.' }) =>
		updateNotification({ id, icon: <CgCheck size={18} />, color: 'green', ...options });
	const fail = (options: NotificationProps = { message: 'There was an error 😔.' }) =>
		updateNotification({ id, icon: <CgClose size={18} />, color: 'red', ...options });

	return [success, fail];
};
