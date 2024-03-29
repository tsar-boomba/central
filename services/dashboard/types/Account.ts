export interface Account {
	id: string;
	createdAt: string;
	updatedAt: string;
	address1: string;
	address2: string | null;
	email: string;
	businessName: string;
	shortName: string;
	city: string;
	zipCode: string;
	phoneNumber: string;
	stripeId: string | null;
	subId: string | null;
	state: string;
}
