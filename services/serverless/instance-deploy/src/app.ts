import fastify from 'fastify';
import { config } from 'dotenv';
import { createEnv } from './createEnv';
import { configLoadBalancer } from './configLoadBalancer';
import { configDomain } from './configDomain';
import fetch from 'node-fetch';

config({ path: '.env.local' });

const API_URI = process.env.API_URI || 'http://localhost:4000';

const app = fastify({
	logger: true,
});

app.post('/', async (req, res) => {
	const jwt = req.headers.jwt;
	if (!jwt) {
		res.statusCode = 400;
		return res.send({ message: 'No key provided.' });
	}
	const { accountId, name, instanceId } = req.body as any;
	const instanceData = await createEnv(name, accountId);

	console.log(instanceData);

	res.send();

	await configLoadBalancer(instanceData);
	const domainInfo = await configDomain(instanceData);

	// call back to main server with info about instance
	await fetch(`${API_URI}/instances/${instanceId}/callback`, {
		headers: { jwt: String(jwt), 'Content-Type': 'application/json' },
		method: 'POST',
		body: JSON.stringify({
			envId: instanceData.EnvironmentId,
			url: domainInfo.name,
			accountId,
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
