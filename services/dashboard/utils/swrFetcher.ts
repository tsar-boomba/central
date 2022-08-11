class ApiError extends Error {
	constructor(private status: number, message?: string) {
		super(message);
	}
}

const fetcher = async <JSON = any>(
	url: string,
	init: RequestInit = { credentials: 'include' },
): Promise<JSON> => {
	const res = await fetch(url, { ...init, credentials: 'include', mode: 'cors' });

	if (!res.ok) {
		const err = new ApiError(
			res.status,
			(await res.json()).message || 'An error occurred while fetching data.',
		);
		throw err;
	}

	return res.json();
};

export default fetcher;
