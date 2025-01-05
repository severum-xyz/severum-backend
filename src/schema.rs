// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
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
    roles (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    user_containers (id) {
        id -> Int4,
        user_id -> Int4,
        container_name -> Uuid,
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
        role_id -> Int4,
    }
}

diesel::joinable!(challenges -> categories (category_id));
diesel::joinable!(user_containers -> users (user_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    categories,
    challenges,
    roles,
    user_containers,
    users,
);
