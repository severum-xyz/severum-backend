// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar
    }
}

diesel::table! {
    challenges (id) {
        id -> Int4,
        category_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        difficulty -> Varchar,
        description -> Text,
        hint -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        pseudo -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Timestamp,
        verified -> Nullable<Bool>,
    }
}

diesel::joinable!(challenges -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    challenges,
    users,
);
