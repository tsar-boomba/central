import { ConfigurationOptionSetting } from '@aws-sdk/client-elastic-beanstalk';

export const createOptionSetter =
	(Namespace: string): ((OptionName: string, Value: string) => ConfigurationOptionSetting) =>
	(OptionName, Value) => ({ Namespace, OptionName, Value });
