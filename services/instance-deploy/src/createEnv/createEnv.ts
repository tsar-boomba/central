import {
	ConfigurationOptionSetting,
	CreateEnvironmentCommand,
	CreateEnvironmentCommandInput,
	CreateEnvironmentCommandOutput,
} from '@aws-sdk/client-elastic-beanstalk';
import { randomBytes } from 'crypto';
import { ebClient } from '../clients';
import { APPLICATION_NAME } from '../constants';
import { getVersions } from '../utils/getVersions';
import { dbOptions } from './dbOptions';
import { envOptions } from './envOptions';

const baseParams: CreateEnvironmentCommandInput = {
	ApplicationName: APPLICATION_NAME,
	SolutionStackName: '64bit Amazon Linux 2 v3.4.16 running Docker',
	Tier: { Name: 'WebServer', Type: 'Standard' },
};

const dbPassOption = (password: string): ConfigurationOptionSetting => ({
	Namespace: 'aws:rds:dbinstance',
	OptionName: 'DBPassword',
	Value: password,
});

const jwtEnvOption = (secret: string): ConfigurationOptionSetting => ({
	Namespace: 'aws:elasticbeanstalk:application:environment',
	OptionName: 'JWT_SECRET',
	Value: secret,
});

/**
 * Create Elastic Beanstalk Environment and return data that will be used later in the deployment process
 * @param accountName
 */
export const createEnv = async (
	accountName: string,
	accountId: string,
): Promise<
	{ dbPass: string; jwtSecret: string; envName: string } & CreateEnvironmentCommandOutput
> => {
	const versions = await getVersions();

	if (!versions.ApplicationVersions) throw new Error('No application versions found!');

	const envName = `${accountName}-${Date.now()}-env`;
	const dbPass = randomBytes(20).toString('hex');
	const jwtSecret = randomBytes(24).toString('hex');

	const params: CreateEnvironmentCommandInput = {
		...baseParams,
		VersionLabel: versions.ApplicationVersions[0].VersionLabel,
		EnvironmentName: envName,
		// Most configuration happens here, db, load balancer, etc...
		OptionSettings: [
			...dbOptions,
			dbPassOption(dbPass),
			...envOptions,
			jwtEnvOption(jwtSecret),
		],
	};

	const createEnvCommand = new CreateEnvironmentCommand(params);

	const res = await ebClient.send(createEnvCommand);

	return { ...res, jwtSecret, dbPass, envName };
};
