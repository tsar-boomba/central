mod resource;
mod role;

#[cfg(feature = "diesel")]
pub use resource::sql_type as resource_sql;
pub use resource::Resource;

#[cfg(feature = "diesel")]
pub use role::sql_type as role_sql;
pub use role::Role;