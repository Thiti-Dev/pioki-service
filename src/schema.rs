// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 32]
        pioki_id -> Varchar,
        #[max_length = 32]
        oauth_display_name -> Varchar,
        #[max_length = 255]
        oauth_profile_picture -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
