mod schema;
pub use schema::users;
pub use schema::users::table;

pub mod model;
pub mod routes;

#[cfg(test)]
pub mod tests;
