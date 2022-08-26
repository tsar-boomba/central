const awsLambdaFastify = require('@fastify/aws-lambda');
const app = require('./app');

const proxy = awsLambdaFastify(app, { serializeLambdaArguments: true });

exports.handler = proxy;
