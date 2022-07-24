import { defineConfig } from 'tsup';

export default defineConfig((options) => ({
	entry: ['src/lambda.ts'],
	outDir: 'dist',
	clean: true,
}));
