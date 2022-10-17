use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SnsMessage {
	pub message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SnsMessageRecord {
	pub sns: SnsMessage
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SnsMessageEvent {
	pub records: [SnsMessageRecord; 0],
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqsMessageRecord {
	pub message_id: String,
	pub body: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SqsMessageEvent {
	pub records: Vec<SqsMessageRecord>,
}

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
	pub env_id: String,
	pub env_name: String,
	pub application_name: String,
}
