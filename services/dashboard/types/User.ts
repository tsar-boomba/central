export enum Resource {
	Load = 'load',
	Carrier = 'carrier',
	Shipper = 'shipper',
}

export enum Role {
	Owner = 'owner',
	Admin = 'admin',
	Moderator = 'moderator',
	User = 'user',
}

export interface User {
	id: string;
	accountId: string;
	username: string;
	firstName: string;
	lastName: string;
	/** Will never be sent to frontend */
	password: string;
	active: boolean;
	instances: string[];
	createPerms: Resource[];
	updatePerms: Resource[];
	deletePerms: Resource[];
	role: Role;
	notes: string | null;
}
