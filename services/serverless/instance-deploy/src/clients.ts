import { ElasticBeanstalkClient } from '@aws-sdk/client-elastic-beanstalk';
import { ElasticLoadBalancingV2Client } from '@aws-sdk/client-elastic-load-balancing-v2';
import { Route53Client } from '@aws-sdk/client-route-53';
import { config } from 'dotenv';

config({ path: '.env.local' });

const credentials = {
	accessKeyId: process.env.ACCESS_ID || '',
	secretAccessKey: process.env.SECRET_ACCESS_ID || '',
};

const defaultConfig = {
	region: 'us-east-1',
	credentials,
};

export const ebClient = new ElasticBeanstalkClient(defaultConfig);

export const elbClient = new ElasticLoadBalancingV2Client(defaultConfig);

export const r53Client = new Route53Client(defaultConfig);
