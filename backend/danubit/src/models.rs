use diesel::prelude::*;
use poem_openapi::{types::multipart, Enum, Multipart, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

// Enumeration Types

#[derive(Serialize, Deserialize, Enum, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ActivityAccess"]
pub enum ActivityAccess {
    Public,
    Members,
    Board,
}

#[derive(Serialize, Deserialize, Enum, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::BoardStatus"]
pub enum BoardStatus {
    False,
    Board,
    ViceChair,
    Chair,
}

#[derive(Serialize, Deserialize, Enum, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::MediaKind"]
pub enum MediaKind {
    Logo,
    Digital,
    Print,
    Screen,
    Banner,
    Extra,
}

// Database and API models

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Identifiable, AsChangeset, Object, Debug,
)]
#[diesel(table_name = crate::schema::activities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub initial_date: PrimitiveDateTime,
    pub is_multi_session: bool,
    pub is_creditable: bool,
    pub is_external: bool,
    pub is_accepted: bool,
    pub is_room_accepted: bool,
    pub is_media_accepted: bool,
    pub is_registration_needed: bool,
    pub access: ActivityAccess,
    pub additional_info: Json,
}

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::activities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewActivity {
    pub name: String,
    pub description: String,
    pub initial_date: PrimitiveDateTime,
    pub is_multi_session: bool,
    pub is_creditable: bool,
    pub is_external: bool,
    pub is_accepted: bool,
    pub is_room_accepted: bool,
    pub is_media_accepted: bool,
    pub is_registration_needed: bool,
    pub access: ActivityAccess,
    pub additional_info: Json,
}

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Identifiable, AsChangeset, Object, Debug,
)]
#[diesel(table_name = crate::schema::asociations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Asociation {
    pub id: Uuid,
    pub short_name: String,
    pub long_name: String,
    pub email: String,
    pub description: String,
    pub is_public_joinable: bool,
    pub info: Json,
    pub manager: Option<i64>,
}

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::asociations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAsociation {
    pub short_name: String,
    pub long_name: String,
    pub email: String,
    pub description: String,
    pub is_public_joinable: bool,
    pub info: Json,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::documents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
    pub id: i64,
    pub asociation: Uuid,
    pub activity: Option<i64>,
    pub name: String,
    pub description: String,
    pub path: String,
    pub creation_date: Date,
    pub is_current: bool,
    pub is_important: bool,
    pub is_manager_accessible: bool,
    pub is_public_accessible: bool,
}

#[derive(AsChangeset, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::documents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DocumentDescription {
    pub asociation: Uuid,
    pub activity: Option<i64>,
    pub name: String,
    pub description: String,
    pub is_current: bool,
    pub is_important: bool,
    pub is_manager_accessible: bool,
    pub is_public_accessible: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::lendings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Lending {
    pub id: i64,
    pub material: i64,
    pub user_id: Uuid,
    pub quantity: i16,
    pub due_date: Date,
}

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Identifiable, AsChangeset, Object, Debug,
)]
#[diesel(table_name = crate::schema::managers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Manager {
    pub id: i64,
    pub user_id: Uuid,
    pub name: String,
    pub contact_email: String,
    pub admin_email: Option<String>,
    pub material_email: Option<String>,
    pub print_email: Option<String>,
    pub comms_email: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::managers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewManager {
    pub user_id: Uuid,
    pub name: String,
    pub contact_email: String,
    pub admin_email: Option<String>,
    pub material_email: Option<String>,
    pub print_email: Option<String>,
    pub comms_email: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, AsChangeset, Object, Debug)]
#[diesel(table_name = crate::schema::materials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Material {
    pub id: i64,
    pub asociation: Uuid,
    pub name: String,
    pub description: String,
    pub quantity: i16,
    pub is_lendable: bool,
}

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::materials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMaterial {
    pub asociation: Uuid,
    pub name: String,
    pub description: String,
    pub quantity: i16,
    pub is_lendable: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i64,
    pub name: String,
    pub activity: i64,
    pub kind: MediaKind,
    pub path: String,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct MediaDescription {
    pub name: String,
    pub activity: i64,
    pub kind: MediaKind,
}

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Identifiable, AsChangeset, Object, Debug,
)]
#[diesel(table_name = crate::schema::members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Member {
    pub id: i64,
    pub user_id: Uuid,
    pub asociation: Uuid,
    pub is_accepted: bool,
    pub accepted_date: Option<Date>,
    pub expiry_date: Option<Date>,
    pub label: Option<String>,
    pub board_status: BoardStatus,
}

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMember {
    pub user_id: Uuid,
    pub asociation: Uuid,
    pub is_accepted: bool,
    pub accepted_date: Option<Date>,
    pub expiry_date: Option<Date>,
    pub label: Option<String>,
    pub board_status: BoardStatus,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct MembershipRequest {
    pub user_id: Uuid,
    pub asociation: Uuid,
    pub is_accepted: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::organizers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organizer {
    pub id: i64,
    pub asociation: Uuid,
    pub activity: i64,
}

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Identifiable, AsChangeset, Object, Debug,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub activated: bool,
    pub password_hash: Option<String>,
    pub additional_info: Json,
}

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub activated: bool,
    pub password_hash: Option<String>,
    pub additional_info: Option<Json>,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct AcceptAction {
    pub accept: bool,
}

#[derive(Multipart, Debug)]
pub struct MediaUpload {
    pub file_data: multipart::JsonField<MediaDescription>,
    pub upload: multipart::Upload,
}

#[derive(Multipart, Debug)]
pub struct DocumentUpload {
    pub file_data: multipart::JsonField<DocumentDescription>,
    pub upload: multipart::Upload,
}
