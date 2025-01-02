// @generated automatically by Diesel CLI.


diesel::table! {
    file (id) {
        id -> Int4,
        owner_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        path -> Varchar,
        #[max_length = 255]
        key -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    file_share (id) {
        id -> Int4,
        file_id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        key -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        nickname -> Varchar,
        #[max_length = 255]
        public_key -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(file -> users (owner_id));
diesel::joinable!(file_share -> file (file_id));
diesel::joinable!(file_share -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    file,
    file_share,
    users,
);
