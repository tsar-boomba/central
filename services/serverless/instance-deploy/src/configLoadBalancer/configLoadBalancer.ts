import { CreateEnvironmentCommandOutput } from '@aws-sdk/client-elastic-beanstalk';
import {
	ActionTypeEnum,
	CreateListenerCommand,
	DescribeListenersCommand,
	DescribeTargetGroupsCommand,
	ModifyListenerCommand,
	RedirectActionStatusCodeEnum,
	SetSecurityGroupsCommand,
} from '@aws-sdk/client-elastic-load-balancing-v2';
import { elbClient } from 'src/clients';
import { httpsListenerParams } from './httpsListener';
import { waitForResources } from './waitForResources';

export const configLoadBalancer = async (instanceData: CreateEnvironmentCommandOutput) => {
	console.log('Waiting for resources...');
	// wait for load balancer and instances to be created
	const balancerArn = await waitForResources(instanceData);

	console.log('Configuring load balancer...');

	// get default listener arn now
	const httpListenerArn = (
		await elbClient.send(
			new DescribeListenersCommand({
				LoadBalancerArn: balancerArn,
			}),
		)
	).Listeners?.[0]?.ListenerArn;

	if (!httpListenerArn) throw new Error("Couldn't find default http listener! 😔");

	// get target group arn
	const targetGroups = await elbClient.send(
		new DescribeTargetGroupsCommand({ LoadBalancerArn: balancerArn }),
	);
	const targetGroupArn = targetGroups.TargetGroups?.[0]?.TargetGroupArn;

	if (!targetGroupArn) throw new Error("Couldn't find target group.");

	// add https listener
	await elbClient.send(
		new CreateListenerCommand(httpsListenerParams({ balancerArn, targetGroupArn })),
	);

	// change default listener to redirect to https
	const modifyListenerCommand = new ModifyListenerCommand({
		ListenerArn: httpListenerArn,
		DefaultActions: [
			{
				Type: ActionTypeEnum.REDIRECT,
				RedirectConfig: {
					StatusCode: RedirectActionStatusCodeEnum.HTTP_301,
					Port: '443',
					Protocol: 'HTTPS',
				},
			},
		],
	});
	await elbClient.send(modifyListenerCommand);

	// finally change security group on balancer to alow https requests
	await elbClient.send(
		new SetSecurityGroupsCommand({
			LoadBalancerArn: balancerArn,
			SecurityGroups: ['sg-0e949ec585c11b34a'],
		}),
	);

	return { balancerArn, targetGroupArn };
};
