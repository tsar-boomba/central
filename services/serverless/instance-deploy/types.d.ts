import 'fastify';

declare module 'fastify' {
	export interface FastifyRequest {
		awsLambda: {
			event: any;
			context: any;
		};
	}
}
