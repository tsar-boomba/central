export const waitFor = async (ms: number) =>
	new Promise<void>((resolve) => setTimeout(resolve, ms));
