mod schema;
pub use schema::instances;
pub use schema::instances::table;

pub mod model;
pub mod routes;

#[cfg(test)]
mod tests;
