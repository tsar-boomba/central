import { Instance, InstanceStatus } from '@/types/Instance';
import { Box } from '@mantine/core';
import { useForm } from '@mantine/form';
import { FormRulesRecord } from '@mantine/form/lib/types';

interface Props {
	create: boolean;
	defaultInstance: Instance;
}

type FormValues = any;

export const instanceValidation: FormRulesRecord<FormValues> = {};

const createDefaultInstance = {
	accountId: '',
	address: '',
	bottomTerms: [],
	businessName: '',
	city: '',
	createdAt: '',
	id: '',
	envId: null,
	key: null,
	name: '',
	phoneNumber: '',
	rateConfEmail: '',
	shortName: '',
	status: InstanceStatus.Deploying,
	topTerms: '',
	updatedAt: '',
	url: '',
	zipCode: '',
};

const InstanceForm = ({ create, defaultInstance = createDefaultInstance }: Props) => {
	const form = useForm<FormValues>({ initialValues: defaultInstance });

	const onSubmit = (data: FormValues) => {};

	return <Box component='form' onSubmit={form.onSubmit(onSubmit)}></Box>;
};

export default InstanceForm;
