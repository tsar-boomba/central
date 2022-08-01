#[cfg(feature = "diesel")]
#[macro_use]
extern crate diesel;

#[macro_use]
mod macros;
mod account;
pub use account::model::*;
#[cfg(feature = "diesel")]
pub use account::schema::*;
mod user;
pub use user::model::*;
#[cfg(feature = "diesel")]
pub use user::schema::*;
mod instance;
pub use instance::model::*;
#[cfg(feature = "diesel")]
pub use instance::schema::*;
pub mod types;

pub trait Model<Id, New, Up, Err> {
    fn find_all() -> Result<Vec<Self>, Err>
    where
        Self: Sized;

    fn find_by_id(target: Id) -> Result<Self, Err>
    where
        Self: Sized;

    fn insert(new: New) -> Result<Self, Err>
    where
        Self: Sized;

    fn update(target: Id, new_vals: Up) -> Result<Self, Err>
    where
        Self: Sized;

    fn delete(target: Id) -> Result<usize, Err>;
}
