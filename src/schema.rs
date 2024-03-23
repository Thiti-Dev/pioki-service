// @generated automatically by Diesel CLI.

diesel::table! {
    friends (pioki_id, pioki_friend_id) {
        #[max_length = 32]
        pioki_id -> Varchar,
        #[max_length = 32]
        pioki_friend_id -> Varchar,
        is_blocked -> Bool,
        #[max_length = 32]
        aka -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 32]
        creator_id -> Varchar,
        origin_quota_limit -> Int4,
        quota_left -> Int4,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    friends,
    posts,
    users,
);
