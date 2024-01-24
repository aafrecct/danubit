// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "activity_access"))]
    pub struct ActivityAccess;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "board_status"))]
    pub struct BoardStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "media_kind"))]
    pub struct MediaKind;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActivityAccess;

    activities (id) {
        id -> Int8,
        #[max_length = 32]
        name -> Varchar,
        description -> Text,
        #[max_length = 32]
        room -> Varchar,
        initial_date -> Timestamp,
        is_multi_session -> Bool,
        is_creditable -> Bool,
        is_external -> Bool,
        is_accepted -> Bool,
        is_room_accepted -> Bool,
        is_media_accepted -> Bool,
        is_registration_needed -> Bool,
        access -> ActivityAccess,
        additional_info -> Jsonb,
    }
}

diesel::table! {
    asociations (id) {
        id -> Uuid,
        #[max_length = 24]
        short_name -> Varchar,
        #[max_length = 128]
        long_name -> Varchar,
        email -> Varchar,
        description -> Text,
        is_public_joinable -> Bool,
        info -> Jsonb,
        manager -> Nullable<Int8>,
        logo -> Nullable<Int8>,
    }
}

diesel::table! {
    documents (id) {
        id -> Int8,
        asociation -> Uuid,
        activity -> Nullable<Int8>,
        #[max_length = 64]
        name -> Varchar,
        description -> Text,
        #[max_length = 128]
        path -> Varchar,
        creation_date -> Date,
        is_current -> Bool,
        is_important -> Bool,
        is_manager_accessible -> Bool,
        is_public_accessible -> Bool,
    }
}

diesel::table! {
    lendings (id) {
        id -> Int8,
        material -> Int8,
        user_id -> Uuid,
        quantity -> Int2,
        due_date -> Date,
    }
}

diesel::table! {
    managers (id) {
        id -> Int8,
        user_id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        contact_email -> Varchar,
        admin_email -> Nullable<Varchar>,
        material_email -> Nullable<Varchar>,
        print_email -> Nullable<Varchar>,
        comms_email -> Nullable<Varchar>,
    }
}

diesel::table! {
    materials (id) {
        id -> Int8,
        asociation -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Text,
        quantity -> Int2,
        available -> Int2,
        is_lendable -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MediaKind;

    media (id) {
        id -> Int8,
        #[max_length = 32]
        name -> Varchar,
        activity -> Int8,
        kind -> MediaKind,
        #[max_length = 128]
        path -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BoardStatus;

    members (id) {
        id -> Int8,
        user_id -> Uuid,
        asociation -> Uuid,
        is_accepted -> Bool,
        accepted_date -> Nullable<Date>,
        expiry_date -> Nullable<Date>,
        #[max_length = 32]
        label -> Nullable<Varchar>,
        board_status -> BoardStatus,
    }
}

diesel::table! {
    organizers (id) {
        id -> Int8,
        asociation -> Uuid,
        activity -> Int8,
        person_in_charge -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 64]
        surname -> Varchar,
        email -> Varchar,
        activated -> Bool,
        #[max_length = 60]
        password_hash -> Nullable<Bpchar>,
        additional_info -> Jsonb,
    }
}

diesel::joinable!(asociations -> managers (manager));
diesel::joinable!(asociations -> media (logo));
diesel::joinable!(documents -> activities (activity));
diesel::joinable!(documents -> asociations (asociation));
diesel::joinable!(lendings -> materials (material));
diesel::joinable!(lendings -> users (user_id));
diesel::joinable!(managers -> users (user_id));
diesel::joinable!(materials -> asociations (asociation));
diesel::joinable!(media -> activities (activity));
diesel::joinable!(members -> asociations (asociation));
diesel::joinable!(members -> users (user_id));
diesel::joinable!(organizers -> activities (activity));
diesel::joinable!(organizers -> asociations (asociation));
diesel::joinable!(organizers -> users (person_in_charge));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    asociations,
    documents,
    lendings,
    managers,
    materials,
    media,
    members,
    organizers,
    users,
);
