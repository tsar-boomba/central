#[macro_export]
/// If using this macro include function in scope named "skip_serialize_pass" with this type signature (value: &String) -> bool.
/// Also, import Role and Resources enums from types
macro_rules! user_models {
    ($parent:ident) => {
        child_model! {
            i32, NaiveDateTime, "users", NewUser, $parent,
            User {
                account_id: String,
                username: String,
                first_name: String,
                last_name: String,
                #[serde(skip_serializing_if = "skip_serialize_pass")]
                password: String,
                active: bool,
                instances: Vec<String>,
                create_perms: Vec<Resource>,
                update_perms: Vec<Resource>,
                delete_perms: Vec<Resource>,
                role: Role,
                notes: Option<String>,
            }
        }
    };
}
