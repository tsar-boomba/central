use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;

pub mod sql_type {
    #[derive(SqlType, Debug, Clone, Copy, Default)]
    #[postgres(type_name = "Resource")]
    pub struct Resource;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[sql_type = "sql_type::Resource"]
#[serde(rename_all = "camelCase")]
pub enum Resource {
    Load,
    Carrier,
    Shipper,
}

impl ToSql<sql_type::Resource, Pg> for Resource {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let t = match *self {
            Resource::Load => "load",
            Resource::Carrier => "carrier",
            Resource::Shipper => "shipper",
        };
        <&str as ToSql<Text, Pg>>::to_sql(&t, out)
    }
}

impl FromSql<sql_type::Resource, Pg> for Resource {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes.expect("Empty resource") {
            b"load" => Ok(Resource::Load),
            b"carrier" => Ok(Resource::Carrier),
            b"shipper" => Ok(Resource::Shipper),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl Into<String> for Resource {
    fn into(self) -> String {
        match self {
            Resource::Load => "load".into(),
            Resource::Carrier => "carrier".into(),
            Resource::Shipper => "shipper".into(),
        }
    }
}

impl From<&str> for Resource {
    fn from(from: &str) -> Self {
        match from {
            "load" => Resource::Load,
            "carrier" => Resource::Carrier,
            "shipper" => Resource::Shipper,
            _ => panic!("{}", &format!("Invalid enum variant: {}", from)),
        }
    }
}
