import { defineConfig } from 'tsup';

export default defineConfig((s) => ({
	entry: ['src/index.ts'],
	outDir: 'dist',
	clean: true,
	noExternal: [/.*/],
	minify: true,
}));
