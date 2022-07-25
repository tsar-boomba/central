use crate::{
    accounts::{self, model::NewAccount},
    api_error::ApiError,
    users::{self, model::NewUser},
};
use models::types::{Role, Resource};
use bcrypt::{hash, DEFAULT_COST};
use diesel::{
    r2d2::{self, Builder, ConnectionManager},
    Connection, PgConnection, RunQueryDsl,
};
use diesel_migrations::embed_migrations;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PoolConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

const POOL_SIZE: u32 = match cfg!(test) {
    true => 1,
    false => 10
};

lazy_static! {
    static ref POOL: DbPool = {
        let db_url = std::env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);

        Builder::new()
            .max_size(POOL_SIZE)
            .build(manager)
            .expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing db.");
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");

    // run migrations before starting test transaction
    embedded_migrations::run(&conn).unwrap();

    if cfg!(test) {
        conn.begin_test_transaction()
            .expect("Failed to start transaction");
    }

    // ensure admin account exists
    diesel::insert_into(accounts::table)
        .values(NewAccount {
            id: "admin".into(),
            address: "admin".into(),
            business_name: "admin".into(),
            city: "admin".into(),
            email: "admin@admin.com".into(),
            phone_number: "000-000-0000".into(),
            short_name: "admin".into(),
            zip_code: "admin".into(),
        })
        .on_conflict_do_nothing()
        .execute(&conn)
        .unwrap();

    let admin_name = std::env::var("ADMIN_USERNAME");
    let admin_pass = std::env::var("ADMIN_PASS");

    if admin_name.is_ok() && admin_pass.is_ok() {
        // add admin user
        diesel::insert_into(users::table)
            .values(NewUser {
                account_id: "admin".into(),
                username: admin_name.unwrap(),
                first_name: "Test".into(),
                last_name: "User".into(),
                password: hash(admin_pass.unwrap(), DEFAULT_COST).unwrap(),
                active: true,
                instances: vec!["hatfield".into()],
                create_perms: vec![Resource::Load, Resource::Carrier, Resource::Shipper],
                update_perms: vec![Resource::Load, Resource::Carrier, Resource::Shipper],
                delete_perms: vec![Resource::Load, Resource::Carrier, Resource::Shipper],
                role: Role::Owner,
                notes: None,
            })
            .on_conflict_do_nothing()
            .execute(&conn)
            .unwrap();
    }
}

pub fn connection() -> Result<PoolConn, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}
