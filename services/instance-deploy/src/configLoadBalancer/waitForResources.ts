import {
	CreateEnvironmentCommandOutput,
	DescribeEnvironmentResourcesCommand,
} from '@aws-sdk/client-elastic-beanstalk';
import { waitFor } from 'src/utils/waitFor';
import { ebClient } from '../clients';

/**
 * Waits for resources to be created, then returns ARN of application load balancer
 * @param instanceData
 * @returns ARN of application load balancer
 */
export const waitForResources = async (instanceData: CreateEnvironmentCommandOutput) => {
	let result: string;

	while (true) {
		const resourcesCommand = new DescribeEnvironmentResourcesCommand({
			EnvironmentId: instanceData.EnvironmentId,
		});
		const resources = await ebClient.send(resourcesCommand);

		const balancerArn = resources.EnvironmentResources?.LoadBalancers?.[0]?.Name;

		// wait for instance to be deployed to start configuring
		if (balancerArn && resources.EnvironmentResources?.Instances?.[0]) {
			result = balancerArn;

			break;
		}

		await waitFor(1000);
	}

	return result;
};
