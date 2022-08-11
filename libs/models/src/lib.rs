#[cfg(feature = "diesel")]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;
mod account;
pub use account::model::*;
#[cfg(feature = "diesel")]
pub use account::schema::*;
mod user;
use regex::Regex;
pub use user::model::*;
#[cfg(feature = "diesel")]
pub use user::schema::*;
mod instance;
pub use instance::model::*;
#[cfg(feature = "diesel")]
pub use instance::schema::*;
pub use validator::{Validate, ValidationError, ValidationErrors};
pub mod types;

// validation regular expressions
lazy_static! {
    static ref ZIP_RE: Regex = Regex::new(r"[\d]{5}(-[\d]{4})?").unwrap();
    static ref EMAIL_RE: Regex = Regex::new(r"^[^\s@]+@([^\s@.,]+\.)+[^\s@.,]{2,}$").unwrap();
    static ref PHONE_RE: Regex =
        Regex::new(r"^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$").unwrap();
}

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

const STATES: [&str; 50] = [
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY",
];

fn validate_state(state: &str) -> Result<(), ValidationError> {
    if crate::STATES.contains(&state) {
        return Ok(());
    }
    Err(ValidationError::new("Invalid two-letter state code."))
}
