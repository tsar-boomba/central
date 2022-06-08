use diesel::table;

table! {
    instances {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        account_id -> Text,
        db_url -> Text,
        url -> Text,
        business_name -> Text,
        short_name -> Text,
        address -> Text,
        city -> Text,
        zip_code -> Text,
        phone_number -> Text,
        rate_conf_email -> Text,
        instance_name -> Nullable<Text>,
        top_terms -> Nullable<Text>,
        bottom_terms -> Nullable<Array<Text>>,
    }
}
