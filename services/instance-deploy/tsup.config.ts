import { defineConfig } from 'tsup';

export default defineConfig((options) => ({
	entry: options.watch ? ['src/app.ts'] : ['src/lambda.ts'],
	outDir: 'dist',
	onSuccess: options.watch ? 'node dist/app.js' : undefined,
	clean: true,
}));
