import fastify from 'fastify';
import { config } from 'dotenv';
import { createEnv } from './createEnv';
import { configLoadBalancer } from './configLoadBalancer';
import { configDomain } from './configDomain';
import fetch from 'node-fetch';
import { verify } from 'jsonwebtoken';

config({ path: '.env.local' });
config();

const API_URI = process.env.API_URI || 'http://localhost:4000';
const JWT_SECRET = process.env.JWT_SECRET || 'thuthy';

const app = fastify({
	logger: true,
});

interface Params {
	instanceId: string;
	accountId: string;
	name: string;
	key: string;
}

app.post('/*', async (req, res) => {
	const jwt = req.headers.jwt;
	if (!jwt) {
		res.statusCode = 400;
		return res.send({ message: 'Not authorized' });
	}

	try {
		verify(String(jwt), JWT_SECRET);
	} catch (e) {
		res.statusCode = 403;
		return res.send({ message: 'Bad token.' });
	}

	let params: Params;

	if (req.awsLambda) {
		// is in lambda mode
		console.log(req.awsLambda.event);
		console.log(req.awsLambda.context);
		console.log(req.body);
		params = req.awsLambda.event;
	} else {
		params = req.body as Params;
	}

	if (!params.key || !params.accountId || !params.name || !params.instanceId) {
		res.statusCode = 400;
		console.log(req.body);
		return res.send({ message: 'Failed to provide a required parameter.' });
	}
	const instanceData = await createEnv(params);

	console.log(instanceData);

	res.send();

	await configLoadBalancer(instanceData);
	const domainInfo = await configDomain(instanceData);

	// call back to main server with info about instance
	await fetch(`${API_URI}/instances/${params.instanceId}/callback`, {
		headers: { jwt: String(jwt), 'Content-Type': 'application/json' },
		method: 'POST',
		body: JSON.stringify({
			envId: instanceData.EnvironmentId,
			url: domainInfo.name,
			accountId: params.accountId,
		}),
	}).then(async (res) => console.log(await res.text()));

	console.log('Configuration done!');
	console.log('Will be available at:', `https://${domainInfo.name}`);
});

if (require.main === module) {
	// called directly i.e. "node app"
	app.listen({ port: 3001 }, (err) => {
		if (err) console.log('Error:', err);
		console.log('server listening on 3001');
	});
} else {
	// required as a module => executed on aws lambda
	module.exports = app;
}
