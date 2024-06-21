// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "page_type"))]
    pub struct PageType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "publish_status"))]
    pub struct PublishStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "sort_type"))]
    pub struct SortType;
}

diesel::table! {
    comm_users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 256]
        nickname -> Varchar,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 256]
        website_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    comms (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        content -> Text,
        comm_user_id -> Nullable<Uuid>,
        blog_user_id -> Nullable<Uuid>,
        note_id -> Nullable<Uuid>,
        page_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    comms_closure (ancestor_id, descendant_id) {
        created_at -> Timestamp,
        updated_at -> Timestamp,
        distance -> Int8,
        ancestor_id -> Uuid,
        descendant_id -> Uuid,
    }
}

diesel::table! {
    info (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        bio -> Text,
        #[max_length = 256]
        title -> Varchar,
    }
}

diesel::table! {
    note_sorts (note_id, sort_id) {
        created_at -> Timestamp,
        updated_at -> Timestamp,
        note_id -> Uuid,
        sort_id -> Uuid,
    }
}

diesel::table! {
    note_tags (note_id, tag_id) {
        created_at -> Timestamp,
        updated_at -> Timestamp,
        note_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PublishStatus;

    notes (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 256]
        title -> Varchar,
        status -> PublishStatus,
        summary -> Text,
        content -> Text,
        views -> Int8,
        comm -> Bool,
        user_id -> Uuid,
        short_id -> Uuid,
    }
}

diesel::table! {
    page_about (id) {
        id -> Uuid,
        #[max_length = 2048]
        avatar_url -> Varchar,
        content -> Text,
        page_id -> Uuid,
    }
}

diesel::table! {
    page_sorts (page_id, sort_id) {
        created_at -> Timestamp,
        updated_at -> Timestamp,
        page_id -> Uuid,
        sort_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PageType;
    use super::sql_types::PublishStatus;

    pages (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        page_type -> PageType,
        status -> PublishStatus,
        comm -> Bool,
        user_id -> Uuid,
        short_id -> Uuid,
    }
}

diesel::table! {
    short_ids (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 16]
        short_name -> Varchar,
        #[max_length = 256]
        subname -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SortType;

    sorts (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 256]
        content -> Varchar,
        sort_order -> Int4,
        sort_type -> Nullable<SortType>,
        parent_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 256]
        content -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 256]
        username -> Varchar,
        #[max_length = 256]
        nickname -> Varchar,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 256]
        password_hash -> Varchar,
    }
}

diesel::joinable!(comms -> comm_users (comm_user_id));
diesel::joinable!(comms -> notes (note_id));
diesel::joinable!(comms -> pages (page_id));
diesel::joinable!(comms -> users (blog_user_id));
diesel::joinable!(note_sorts -> notes (note_id));
diesel::joinable!(note_sorts -> sorts (sort_id));
diesel::joinable!(note_tags -> notes (note_id));
diesel::joinable!(note_tags -> tags (tag_id));
diesel::joinable!(notes -> short_ids (short_id));
diesel::joinable!(notes -> users (user_id));
diesel::joinable!(page_about -> pages (page_id));
diesel::joinable!(page_sorts -> pages (page_id));
diesel::joinable!(page_sorts -> sorts (sort_id));
diesel::joinable!(pages -> short_ids (short_id));
diesel::joinable!(pages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comm_users,
    comms,
    comms_closure,
    info,
    note_sorts,
    note_tags,
    notes,
    page_about,
    page_sorts,
    pages,
    short_ids,
    sorts,
    tags,
    users,
);
