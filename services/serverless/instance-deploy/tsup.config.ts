import { defineConfig } from 'tsup';

export default defineConfig((os) => ({
	entry: ['src/index.ts'],
	outDir: 'dist',
	clean: true,
	noExternal: [/.*/],
	minify: true,
}));
