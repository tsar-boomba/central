import { createOptionSetter } from 'src/utils/createOptionSetter';

const setDb = createOptionSetter('aws:rds:dbinstance');

const dbStorage = setDb('DBAllocatedStorage', '10');

const dbDeletion = setDb('DBDeletionPolicy', 'Delete');

const dbEngine = setDb('DBEngine', 'postgres');

const dbEngineVer = setDb('DBEngineVersion', '14.2');

const dbUser = setDb('DBUser', 'backend');

const dbCoupling = setDb('HasCoupledDatabase', 'true');

const dbInstance = setDb('DBInstanceClass', 'db.t4g.micro');

export const dbOptions = Object.values({
	dbStorage,
	dbDeletion,
	dbEngine,
	dbEngineVer,
	dbUser,
	dbCoupling,
	dbInstance,
});
