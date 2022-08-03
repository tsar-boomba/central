import fastify from 'fastify';
import { config } from 'dotenv';
import { createEnv } from './createEnv';
import { configLoadBalancer } from './configLoadBalancer';
import { configDomain } from './configDomain';

config({ path: '.env.local' });

const app = fastify({});

app.post('/', async (req, res) => {
	const instanceData = await createEnv('hatfield', 'SgUi7_d');

	console.log(instanceData);

	res.send(instanceData);

	await configLoadBalancer(instanceData);
	const domainInfo = await configDomain(instanceData);
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
