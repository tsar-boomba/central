import { defineConfig } from 'tsup';

export default defineConfig((ss) => ({
	entry: ['src/index.ts'],
	outDir: 'dist',
	clean: true,
	noExternal: [/.*/],
	minify: true,
}));
