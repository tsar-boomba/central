export type ProcessOutput = {
	isSuccess: boolean;
	isFail: boolean;
	status?: Deno.ProcessStatus;
	/** Must use `pipeOutputs` or `throwOnErr` option to access this */
	stdOut?: string;
	/** Must use `pipeOutputs` or `throwOnErr` option to access this */
	stdErr?: string;
};

export type ExecOptions = {
	throwOnErr?: boolean;
	workingDirectory?: string;
	pipeOutputs?: boolean;
};

const textDecoder = new TextDecoder();

/**
 * Run a command
 */
export const run = async (
	command: string[],
	{ throwOnErr, workingDirectory, pipeOutputs }: ExecOptions = {}
): Promise<ProcessOutput> => {
	const process = Deno.run({
		cmd: command,
		cwd: workingDirectory,
		stdout: throwOnErr ? 'piped' : pipeOutputs ? 'piped' : undefined,
		stderr: throwOnErr ? 'piped' : pipeOutputs ? 'piped' : undefined,
	});
	const [rawStatus, rawStdOut, rawStdErr] = await Promise.allSettled([
		process.status(),
		process.output(),
		process.stderrOutput(),
	]);

	process.close();

	const status = rawStatus.status === 'fulfilled' ? rawStatus.value : undefined;
	const stdOut =
		rawStdOut.status === 'fulfilled' ? textDecoder.decode(rawStdOut.value) : undefined;
	const stdErr =
		rawStdErr.status === 'fulfilled' ? textDecoder.decode(rawStdErr.value) : undefined;

	if (throwOnErr && (!status || !status?.success)) {
		throw new Error(`Executing command: '${command}' failed with ${stdErr}`);
	}

	return {
		status,
		stdOut,
		stdErr,
		isSuccess: status?.success ?? false,
		isFail: !(status?.success ?? false),
	};
};

export const commandExists = async (command: string): Promise<boolean> =>
	(await run(['command', '-v', command])).isSuccess;
