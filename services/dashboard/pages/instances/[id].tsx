import { Role } from '@/types/User';
import { DEFAULT_SSR } from '@/utils/authUtils';

const Instance = () => {};

export const getServerSideProps = DEFAULT_SSR('/instances', Role.Admin);

export default Instance;
