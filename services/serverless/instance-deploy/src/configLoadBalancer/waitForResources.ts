import {
	CreateEnvironmentCommandOutput,
	DescribeEnvironmentResourcesCommand,
	TerminateEnvironmentCommand,
} from '@aws-sdk/client-elastic-beanstalk';
import { waitFor } from 'src/utils/waitFor';
import { ebClient } from '../clients';

// ms till give up on env
const TIMEOUT = 14 * 1000 * 60;

/**
 * Waits for resources to be created, then returns ARN of application load balancer
 * @param instanceData
 * @returns ARN of application load balancer
 */
export const waitForResources = async (instanceData: CreateEnvironmentCommandOutput) => {
	let result: string;
	const startTime = Date.now();

	while (true) {
		const resourcesCommand = new DescribeEnvironmentResourcesCommand({
			EnvironmentId: instanceData.EnvironmentId,
		});
		if (Date.now() - startTime >= TIMEOUT) {
			await ebClient.send(
				new TerminateEnvironmentCommand({
					EnvironmentId: instanceData.EnvironmentId,
					ForceTerminate: true,
					TerminateResources: true,
				}),
			);
			throw new Error('Timed out waiting for environment resources.');
		}
		const resources = await ebClient.send(resourcesCommand);

		const balancerArn = resources.EnvironmentResources?.LoadBalancers?.[0]?.Name;

		// wait for instance to be deployed to start configuring
		if (balancerArn && resources.EnvironmentResources?.Instances?.[0]) {
			result = balancerArn;

			break;
		}

		await waitFor(5000);
	}

	return result;
};
