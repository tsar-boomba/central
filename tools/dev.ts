#!/usr/bin/env -S deno run -A

import { parse } from 'https://deno.land/std@0.160.0/flags/mod.ts';
import { run, commandExists } from './exec.ts';
const { watch } = parse(Deno.args, { alias: { w: 'watch' } });

// check for required commands
if (!(await commandExists('docker'))) {
	console.log('You need to install docker to use this script.');
	Deno.exit(1);
}
if (!(await commandExists('cargo'))) {
	console.log('You need to install rust to use this script.');
	Deno.exit(1);
}
if (!(await commandExists('node'))) {
	console.log('You need to install node.js to use this script.');
	Deno.exit(1);
}

if ((await run(['docker', 'ps', '-a'], { pipeOutputs: true })).stdOut?.includes('central-dev-db')) {
	run(['docker', 'start', 'central-dev-db'], { throwOnErr: true });
} else {
	run(
		[
			'docker',
			'run',
			'--name',
			'central-dev-db',
			'-p',
			'5431:5432',
			'--env-file',
			'./.db.env',
			'-d',
			'postgres:14.4-alpine',
		],
		{ throwOnErr: true }
	);
}

const cargoCommand = watch
	? (_pkg: string) => ['cargo', 'watch', '-x', 'run']
	: (pkg: string) => ['cargo', 'run', '-p', pkg];

run(cargoCommand('crud'), { workingDirectory: 'services/crud' });
run(cargoCommand('payments'), { workingDirectory: 'services/payments' });
run(cargoCommand('gateway'), { workingDirectory: 'services/gateway' });
run(['pnpm', 'dev'], { workingDirectory: 'services/dashboard' });

const cleanup = () => {
	run(['docker', 'stop', 'central-dev-db'], { throwOnErr: true });
};

Deno.addSignalListener('SIGTERM', cleanup);
Deno.addSignalListener('SIGINT', cleanup);
