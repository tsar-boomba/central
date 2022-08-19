import {
	ConfigurationOptionSetting,
	CreateEnvironmentCommand,
	CreateEnvironmentCommandInput,
	CreateEnvironmentCommandOutput,
	ListAvailableSolutionStacksCommand,
} from '@aws-sdk/client-elastic-beanstalk';
import { nanoid } from 'nanoid';
import { Params } from 'src/app';
import { createOptionSetter } from 'src/utils/createOptionSetter';
import { ebClient } from '../clients';
import { APPLICATION_NAME } from '../constants';
import { getVersions } from '../utils/getVersions';
import { dbOptions } from './dbOptions';
import { envOptions } from './envOptions';

const baseParams: CreateEnvironmentCommandInput = {
	ApplicationName: APPLICATION_NAME,
	Tier: { Name: 'WebServer', Type: 'Standard' },
};

const setEnvVar = createOptionSetter('aws:elasticbeanstalk:application:environment');

const dbPassOption = (password: string): ConfigurationOptionSetting => ({
	Namespace: 'aws:rds:dbinstance',
	OptionName: 'DBPassword',
	Value: password,
});

/**
 * Create Elastic Beanstalk Environment and return data that will be used later in the deployment process
 * @param accountName
 */
export const createEnv = async ({
	name,
	accountId,
	key,
	instanceId,
}: Params): Promise<
	{ dbPass: string; jwtSecret: string; envName: string } & CreateEnvironmentCommandOutput
> => {
	const versions = await getVersions();
	const solutionStacks = await ebClient.send(new ListAvailableSolutionStacksCommand({}));
	const solutionStack = solutionStacks.SolutionStacks?.find((stack) =>
		stack.includes('running Docker'),
	);

	if (!solutionStack) throw new Error('No solution stack was found');

	if (!versions.ApplicationVersions) throw new Error('No application versions found!');

	const envName = `${name}-${accountId}`;
	const dbPass = nanoid(36);
	const jwtSecret = nanoid(36);

	const params: CreateEnvironmentCommandInput = {
		...baseParams,
		VersionLabel: versions.ApplicationVersions[0].VersionLabel,
		EnvironmentName: envName,
		SolutionStackName: solutionStack,
		// Most configuration happens here, db, load balancer, etc...
		OptionSettings: [
			...dbOptions,
			dbPassOption(dbPass),
			...envOptions,
			setEnvVar('JWT_SECRET', jwtSecret),
			setEnvVar('KEY', key),
			setEnvVar('ID', instanceId),
		],
	};

	const createEnvCommand = new CreateEnvironmentCommand(params);

	const res = await ebClient.send(createEnvCommand);

	return { ...res, jwtSecret, dbPass, envName };
};
