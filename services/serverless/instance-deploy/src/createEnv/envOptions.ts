import { ConfigurationOptionSetting } from '@aws-sdk/client-elastic-beanstalk';
import { createOptionSetter } from 'src/utils/createOptionSetter';

const setEnv = createOptionSetter('aws:elasticbeanstalk:environment');

const balancerType = setEnv('LoadBalancerType', 'application');

const deploymentPolicy: ConfigurationOptionSetting = {
	Namespace: 'aws:elasticbeanstalk:command',
	OptionName: 'DeploymentPolicy',
	Value: 'RollingWithAdditionalBatch',
};

const instanceType: ConfigurationOptionSetting = {
	Namespace: 'aws:ec2:instances',
	OptionName: 'InstanceTypes',
	Value: 't2.micro,t3.micro',
};

const launchIamProfile: ConfigurationOptionSetting = {
	Namespace: 'aws:autoscaling:launchconfiguration',
	OptionName: 'IamInstanceProfile',
	Value: 'aws-elasticbeanstalk-ec2-role',
};

// security group to allow requests from load balancer
const securityGroup: ConfigurationOptionSetting = {
	Namespace: 'aws:autoscaling:launchconfiguration',
	OptionName: 'SecurityGroups',
	Value: 'instances',
};

// key pair for sshing if needed
const ec2KeyPair: ConfigurationOptionSetting = {
	Namespace: 'aws:autoscaling:launchconfiguration',
	OptionName: 'EC2KeyName',
	Value: 'eb-debug',
};

const setDefaultProcess = createOptionSetter('aws:elasticbeanstalk:environment:process:default');

const healthCheckPath = setDefaultProcess('HealthCheckPath', '/health');

const healthCheckCode = setDefaultProcess('MatcherHTTPCode', '308,301,307,302');

export const envOptions = Object.values({
	balancerType,
	deploymentPolicy,
	instanceType,
	launchIamProfile,
	securityGroup,
	ec2KeyPair,
	healthCheckPath,
	healthCheckCode,
});
