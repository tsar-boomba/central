mod schema;
pub use schema::accounts;
pub use schema::accounts::table;

pub mod model;
pub mod routes;

#[cfg(test)]
mod tests;
