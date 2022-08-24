import { defineConfig } from 'tsup';

export default defineConfig((o) => ({
	entry: ['src/index.ts'],
	outDir: 'dist',
	clean: true,
	noExternal: [/.*/],
	minify: true,
}));
