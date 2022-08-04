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
				// milkyweb.app cert
				'arn:aws:acm:us-east-1:262246349843:certificate/67e2142a-df92-424b-b92c-f5af04d12952',
		},
	],
	SslPolicy: 'ELBSecurityPolicy-2016-08',
});
