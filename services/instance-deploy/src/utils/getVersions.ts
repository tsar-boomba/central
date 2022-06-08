import { DescribeApplicationVersionsCommand } from '@aws-sdk/client-elastic-beanstalk';
import { ebClient } from '../clients';
import { APPLICATION_NAME } from '../constants';

export const getVersions = async () =>
	ebClient.send(
		new DescribeApplicationVersionsCommand({
			ApplicationName: APPLICATION_NAME,
		}),
	);
