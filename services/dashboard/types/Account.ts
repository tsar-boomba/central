export interface Account {
	id: string;
	createdAt: string;
	updatedAt: string;
	address: string;
	email: string;
	businessName: string;
	shortName: string;
	city: string;
	zipCode: string;
	phoneNumber: string;
	/** Never will be sent by or to frontend */
	stripeId: string | null;
	state: string;
}
