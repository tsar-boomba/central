use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployMessage {
    pub account_id: String,
    pub instance_id: String,
    pub name: String,
    pub key: String,
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMessage {
    pub account_id: String,
    pub instance_id: String,
    pub name: String,
    pub key: String,
    pub jwt: String,
    pub env_id: String,
    pub env_name: String,
    pub application_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailMessage {
	pub account_id: String,
    pub instance_id: String,
    pub name: String,
    pub key: String,
    pub jwt: String,
    #[serde(default)]
	pub env_id: Option<String>,
    #[serde(default)]
	pub env_name: Option<String>,
}
