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
	name: string;
	status: InstanceStatus;
	//#[serde(skip)]
	key: string | null;
	//#[serde(skip)]
	envId: string | null;
	url: string | null;

	// rate conf stuff
	businessName: string;
	shortName: string;
	address1: string;
	address2: string | null;
	state: string;
	city: string;
	zipCode: string;
	phoneNumber: string;
	email: string;
	/** html which will be added to invoice */
	topText: string | null;
	// same as above
	bottomText: string | null;
}
