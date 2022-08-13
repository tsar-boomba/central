export enum InstanceStatus {
	Deploying = 'deploying',
	Failed = 'failed',
	Ok = 'ok',
	Unhealthy = 'unhealthy',
	Inactive = 'inactive',
	Configured = 'configured',
}

export interface Instance {
	id: string;
	createdAt: string;
	updatedAt: string;
	accountId: string;
	businessName: string;
	shortName: string;
	address: string;
	city: string;
	zipCode: string;
	phoneNumber: string;
	name: string;
	status: InstanceStatus;
	//#[serde(skip)]
	key: string | null;
	//#[serde(skip)]
	envId: string | null;
	url: string | null;
	// rate conf stuff
	rateConfEmail: string;
	topTerms: string | null;
	bottomTerms: string[] | null;
}
