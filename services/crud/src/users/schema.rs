use diesel::table;

table! {
    use diesel::sql_types::*;
    use models::types::resource_sql::Resource;
    use models::types::role_sql::Role;

    users {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        account_id -> Text,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        password -> Text,
        active -> Bool,
        instances -> Array<Text>,
        create_perms -> Array<Resource>,
        update_perms -> Array<Resource>,
        delete_perms -> Array<Resource>,
        role -> Role,
        notes -> Nullable<Text>,
    }
}
