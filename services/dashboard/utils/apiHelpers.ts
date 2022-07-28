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
export const callApi = ({
	route,
	body,
	method = 'POST',
}: {
	route: string;
	body?: unknown;
	method?: 'POST' | 'DELETE' | 'PATCH';
}) => {
	if (!method) method = 'POST';
	const response = fetch(api(route), {
		method,
		body: JSON.stringify(body),
		credentials: 'include',
		headers: {
			'Content-Type': 'application/json',
		},
	});
	return response;
};
