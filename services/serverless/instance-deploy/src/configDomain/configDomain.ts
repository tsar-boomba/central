import {
	CreateEnvironmentCommandOutput,
	DescribeEnvironmentsCommand,
} from '@aws-sdk/client-elastic-beanstalk';
import { ChangeResourceRecordSetsCommand } from '@aws-sdk/client-route-53';
import { ebClient, r53Client } from 'src/clients';
import { APPLICATION_NAME, DOMAIN_NAME, HOSTED_ZONE_ID } from 'src/constants';

export const configDomain = async (instanceData: CreateEnvironmentCommandOutput) => {
	console.log('Configuring domain record...');

	// get environment url
	const envs = await ebClient.send(
		new DescribeEnvironmentsCommand({
			EnvironmentIds: [instanceData.EnvironmentId || ''],
			ApplicationName: APPLICATION_NAME,
			IncludeDeleted: false,
		}),
	);

	console.log(JSON.stringify(envs, null, 4));

	const CNAME = envs.Environments?.[0]?.CNAME;

	if (!CNAME) throw new Error('Could not find CNAME for environment!');

	const url = `${instanceData.EnvironmentName}.${DOMAIN_NAME}`;

	// Create A record for eb environment
	const createRecordCommand = new ChangeResourceRecordSetsCommand({
		HostedZoneId: HOSTED_ZONE_ID,
		ChangeBatch: {
			Changes: [
				{
					Action: 'CREATE',
					ResourceRecordSet: {
						Type: 'A',
						Name: url,
						AliasTarget: {
							DNSName: CNAME,
							// us east 1
							HostedZoneId: 'Z117KPS5GTRQ2G',
							EvaluateTargetHealth: false,
						},
					},
				},
			],
		},
	});

	console.log('Requesting new record...');
	const change = r53Client.send(createRecordCommand);
	change.catch((err) => console.log(err));
	await change;

	return { name: url, cname: CNAME };
};
