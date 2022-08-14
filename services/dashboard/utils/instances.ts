import { InstanceStatus } from '@/types/Instance';
import {
	IconCircleCheck,
	IconCircleMinus,
	IconCircleX,
	IconRefresh,
	IconSettings,
	TablerIcon,
} from '@tabler/icons';

export const instanceStatusToText = (status: InstanceStatus) => {
	switch (status) {
		case InstanceStatus.Ok:
			return 'Ok';
		case InstanceStatus.Deploying:
			return 'Deploying';
		case InstanceStatus.Configured:
			return 'Configured';
		case InstanceStatus.Inactive:
			return 'Inactive';
		case InstanceStatus.Failed:
			return 'Deployment Failed';
		case InstanceStatus.Unhealthy:
			return 'Unhealthy';
		default:
			return 'Unknown Status';
	}
};

export const instanceStatusToIcon = (status: InstanceStatus): TablerIcon => {
	switch (status) {
		case InstanceStatus.Ok:
			return IconCircleCheck;
		case InstanceStatus.Deploying:
			return IconRefresh;
		case InstanceStatus.Configured:
			return IconSettings;
		case InstanceStatus.Inactive:
			return IconCircleMinus;
		case InstanceStatus.Failed:
			return IconCircleX;
		case InstanceStatus.Unhealthy:
			return IconCircleX;
		default:
			return IconCircleMinus;
	}
};
