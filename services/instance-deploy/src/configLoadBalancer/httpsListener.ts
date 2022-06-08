import {
	ActionTypeEnum,
	CreateListenerCommandInput,
	ProtocolEnum,
} from '@aws-sdk/client-elastic-load-balancing-v2';

interface Params {
	balancerArn: string;
	targetGroupArn: string;
}

export const httpsListenerParams = ({
	balancerArn,
	targetGroupArn,
}: Params): CreateListenerCommandInput => ({
	LoadBalancerArn: balancerArn,
	Port: 443,
	Protocol: 'HTTPS' as ProtocolEnum,
	DefaultActions: [
		{
			Type: ActionTypeEnum.FORWARD,
			TargetGroupArn: targetGroupArn,
		},
	],
	Certificates: [
		{
			CertificateArn:
				// TODO: is currently igamble ssl cert
				'arn:aws:acm:us-east-1:740633958367:certificate/429a2d02-a978-4b41-b9b7-f0c9f317cfb4',
		},
	],
	SslPolicy: 'ELBSecurityPolicy-2016-08',
});
