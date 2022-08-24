import { defineConfig } from 'tsup';

export default defineConfig((ost) => ({
	entry: ['src/index.ts'],
	outDir: 'dist',
	clean: true,
	noExternal: [/.*/],
	minify: true,
}));
