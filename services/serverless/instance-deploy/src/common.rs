use lazy_static::lazy_static;

#[allow(dead_code)]
pub const DOMAIN_NAME: &'static str = "milkyweb.app";
#[allow(dead_code)]
pub const HOSTED_ZONE_ID: &'static str = "Z0898550109O7ZB98C1FF";
#[allow(dead_code)]
pub const ELB_ZONE_ID: &'static str = "Z117KPS5GTRQ2G";

lazy_static! {
    pub static ref CRUD_URI: String = std::env::var("CRUD_URI").unwrap();
}
