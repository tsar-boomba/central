/* eslint-disable @typescript-eslint/no-var-requires */
const withBundleAnalyzer = require('@next/bundle-analyzer')({
	enabled: process.env.ANALYZE === 'true',
});

/** @type {import('next').NextConfig} */
module.exports = withBundleAnalyzer({
	reactStrictMode: true,
	swcMinify: true,
	webpackDevMiddleware: (config) => {
		config.watchOptions = {
			poll: 1000,
			aggregateTimeout: 300,
			ignored: ['**/node_modules'],
		};
		return config;
	},
});
