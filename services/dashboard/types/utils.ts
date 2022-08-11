import { Account } from './Account';
import { User } from './User';

export type RegisterAccount = Omit<
	Account,
	'id' | 'createdAt' | 'updatedAt' | 'stripeId' | 'subId'
>;
export type RegisterUser = Pick<User, 'firstName' | 'lastName' | 'password' | 'username'> & {
	confirmPass: string;
};

export type NewAccount = Omit<Account, 'id' | 'createdAt' | 'updatedAt' | 'stripeId' | 'subId'>;
export type NewUser = Omit<User, 'id' | 'createdAt' | 'updatedAt' | 'accountId' | 'notes'>;

export type UpdateAccount = Omit<Account, 'id' | 'createdAt' | 'updatedAt' | 'stripeId' | 'subId'>;
