use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;

pub mod sql_type {
    #[derive(SqlType, Debug, Clone, Copy, Default)]
    #[postgres(type_name = "Role")]
    pub struct Role;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[sql_type = "sql_type::Role"]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Owner,
    Admin,
    Moderator,
    User,
}

impl ToSql<sql_type::Role, Pg> for Role {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let t = match *self {
            Role::Owner => "owner",
            Role::Admin => "admin",
            Role::Moderator => "moderator",
            Role::User => "user",
        };
        <&str as ToSql<Text, Pg>>::to_sql(&t, out)
    }
}

impl FromSql<sql_type::Role, Pg> for Role {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes.expect("Empty resource") {
            b"owner" => Ok(Role::Owner),
            b"admin" => Ok(Role::Admin),
            b"moderator" => Ok(Role::Moderator),
            b"user" => Ok(Role::User),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<Role> for String {
    fn from(role: Role) -> Self {
        match role {
            Role::Owner => "owner".into(),
            Role::Admin => "admin".into(),
            Role::Moderator => "moderator".into(),
            Role::User => "user".into(),
        }
    }
}

impl From<&str> for Role {
    fn from(from: &str) -> Self {
        match from {
            "owner" => Role::Owner,
            "admin" => Role::Admin,
            "moderator" => Role::Moderator,
            "user" => Role::User,
            _ => panic!("{}", &format!("Invalid enum variant: {}", from)),
        }
    }
}
