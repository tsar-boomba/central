import { NotificationsProvider as MantineNotificationsProvider } from '@mantine/notifications';
import { memo } from 'react';

const NotificationsProvider: React.FC<Partial<any>> = ({ children, ...props }) => {
	return <MantineNotificationsProvider {...props}>{children}</MantineNotificationsProvider>;
};

export default memo(NotificationsProvider);
