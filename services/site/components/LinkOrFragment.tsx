import Link from 'next/link';
import { PropsWithChildren } from 'react';

export const LinkOrFragment: React.FC<PropsWithChildren<{ href?: string; cond: boolean }>> = ({
	children,
	href,
	cond,
}) =>
	cond ? (
		<Link href={href ?? '/'} passHref>
			{children}
		</Link>
	) : (
		<>{children}</>
	);
