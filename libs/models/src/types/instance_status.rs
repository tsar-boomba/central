#[cfg(feature = "diesel")]
use diesel::deserialize::{self, FromSql};
#[cfg(feature = "diesel")]
use diesel::pg::Pg;
#[cfg(feature = "diesel")]
use diesel::serialize::{self, Output, ToSql};
#[cfg(feature = "diesel")]
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
#[cfg(feature = "diesel")]
use std::io::Write;

#[cfg(feature = "diesel")]
pub mod sql_type {
    #[derive(SqlType, Debug, Clone, Copy, Default)]
    #[postgres(type_name = "InstanceStatus")]
    pub struct InstanceStatus;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "diesel", sql_type = "sql_type::InstanceStatus")]
#[serde(rename_all = "camelCase")]
pub enum InstanceStatus {
    /// Deployment started, domain & load balancer not yet configured
    Deploying,
    /// Deployment failed
	Failed,
    /// Deployed and operating correctly
    Ok,
    /// Deployed and not operating correctly
    Unhealthy,
    /// Not deployed
    Inactive,
    /// Deploying, but domain & load balancer are configured
    Configured
}

#[cfg(feature = "diesel")]
impl ToSql<sql_type::InstanceStatus, Pg> for InstanceStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let t = match *self {
            InstanceStatus::Deploying => "deploying",
			InstanceStatus::Failed => "failed",
            InstanceStatus::Ok => "ok",
            InstanceStatus::Unhealthy => "unhealthy",
            InstanceStatus::Inactive => "inactive",
            InstanceStatus::Configured => "configured",
        };
        <&str as ToSql<Text, Pg>>::to_sql(&t, out)
    }
}

#[cfg(feature = "diesel")]
impl FromSql<sql_type::InstanceStatus, Pg> for InstanceStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes.expect("Empty resource") {
            b"deploying" => Ok(InstanceStatus::Deploying),
            b"failed" => Ok(InstanceStatus::Failed),
            b"ok" => Ok(InstanceStatus::Ok),
			b"unhealthy" => Ok(InstanceStatus::Unhealthy),
            b"inactive" => Ok(InstanceStatus::Inactive),
            b"configured" => Ok(InstanceStatus::Configured),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl Into<String> for InstanceStatus {
    fn into(self) -> String {
        match self {
            InstanceStatus::Deploying => "deploying".into(),
			InstanceStatus::Failed => "failed".into(),
            InstanceStatus::Ok => "ok".into(),
            InstanceStatus::Unhealthy => "unhealthy".into(),
            InstanceStatus::Inactive => "inactive".into(),
            InstanceStatus::Configured => "configured".into(),
        }
    }
}

impl From<&str> for InstanceStatus {
    fn from(from: &str) -> Self {
        match from {
            "deploying" => InstanceStatus::Deploying,
            "failed" => InstanceStatus::Failed,
            "ok" => InstanceStatus::Ok,
			"unhealthy" => InstanceStatus::Unhealthy,
            "inactive" => InstanceStatus::Inactive,
            "configured" => InstanceStatus::Configured,
            _ => panic!("{}", &format!("Invalid enum variant: {}", from)),
        }
    }
}
