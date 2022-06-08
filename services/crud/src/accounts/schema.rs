use diesel::table;

table! {
    accounts {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        address -> Text,
        email -> Text,
        business_name -> Text,
        short_name -> Text,
        city -> Text,
        zip_code -> Text,
        phone_number -> Text,
    }
}
