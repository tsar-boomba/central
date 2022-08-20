import { GetServerSidePropsContext } from 'next';

export const api = (route: string) =>
	(typeof window === 'undefined' ? process.env.SERVER_API_URL : process.env.NEXT_PUBLIC_API_URL) +
	route;

export const ssrFetch = (
	route: string,
	{ req }: GetServerSidePropsContext,
	opts: RequestInit = {},
) => fetch(route, { ...opts, headers: { ...opts.headers, Cookie: `at=${req.cookies.at}` } });

/**
 *
 * @description Meant for all NON-GET requests, which will be handled by swr
 * @returns Promise with Response object
 */
export const callApi = <Body extends unknown>({
	route,
	body,
	method = 'POST',
}: {
	route: string;
	body?: Body;
	method?: 'POST' | 'DELETE' | 'PATCH' | 'PUT';
}) => {
	if (!method) method = 'POST';
	const response = fetch(api(route), {
		method,
		body: JSON.stringify(body),
		mode: 'cors',
		credentials: 'include',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return response;
};

export const resError = (
	jsonPromise: Promise<any>,
	defaultErr = 'An error ocurred.',
): Promise<string> =>
	jsonPromise.then((json) => json?.message || defaultErr).catch(() => defaultErr);

export const isNotFound = (res: Response) => res.status === 404;

export const isServerError = (res: Response) => res.status >= 500;
