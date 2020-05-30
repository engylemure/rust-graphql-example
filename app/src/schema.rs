table! {
    auth_assignments (item_name, user_id) {
        item_name -> Varchar,
        user_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    auth_items (name) {
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Smallint,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    auth_item_children (parent, child) {
        parent -> Varchar,
        child -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::external_user_provider::UserProviderMapping;
    external_user_providers (id) {
        id -> Varchar,
        user_id -> Varchar,
        external_id -> Varchar,
        email -> Nullable<Varchar>,
        provider -> UserProviderMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        hash -> Blob,
        salt -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

table! {
    user_tokens (id) {
        id -> Varchar,
        token -> Mediumtext,
        refresh_token -> Mediumtext,
        user_id -> Varchar,
        refresh_expire_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(auth_assignments -> auth_items (item_name));
joinable!(external_user_providers -> users (user_id));
joinable!(user_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    auth_assignments,
    auth_items,
    auth_item_children,
    external_user_providers,
    users,
    user_tokens,
);
