use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::cmp::{PartialEq, PartialOrd};
use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

// Enumeration Types

#[derive(Serialize, Deserialize, PartialEq, Enum, DbEnum, Debug)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::schema::sql_types::ActivityAccess"]
pub enum ActivityAccess {
    Public,
    Members,
    Board,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Enum, DbEnum, Debug)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::schema::sql_types::BoardStatus"]
pub enum BoardStatus {
    False,
    Board,
    ViceChair,
    Chair,
}

#[derive(Serialize, Deserialize, PartialEq, Enum, DbEnum, Debug)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::schema::sql_types::MediaKind"]
pub enum MediaKind {
    Logo,
    Digital,
    Print,
    Screen,
    Banner,
    Extra,
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
pub struct NaiveUser {
    pub username: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub activated: bool,
    pub password_hash: Option<String>,
    pub additional_info: Option<Json>,
}

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Identifiable, AsChangeset, Object, Debug,
)]
#[diesel(table_name = crate::schema::activities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub room: String,
    pub initial_date: PrimitiveDateTime,
    pub duration: i32,
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
pub struct NaiveActivity {
    pub name: String,
    pub description: String,
    pub room: String,
    pub initial_date: PrimitiveDateTime,
    pub is_multi_session: bool,
    pub is_creditable: bool,
    pub is_external: bool,
    pub is_accepted: bool,
    pub is_room_accepted: bool,
    pub is_media_accepted: bool,
    pub is_registration_needed: bool,
    pub access: ActivityAccess,
    pub additional_info: Option<Json>,
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
    pub logo: Option<i64>,
}

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::asociations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NaiveAsociation {
    pub short_name: String,
    pub long_name: String,
    pub email: String,
    pub description: String,
    pub is_public_joinable: bool,
    pub info: Option<Json>,
    pub manager: Option<i64>,
    pub logo: Option<i64>,
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
pub struct NaiveManager {
    pub user_id: Uuid,
    pub name: String,
    pub contact_email: String,
    pub admin_email: Option<String>,
    pub material_email: Option<String>,
    pub print_email: Option<String>,
    pub comms_email: Option<String>,
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
pub struct NaiveDocument {
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

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Identifiable, AsChangeset, Object, Debug,
)]
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

#[derive(Insertable, Serialize, AsChangeset, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::materials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NaiveMaterial {
    pub asociation: Uuid,
    pub name: String,
    pub description: String,
    pub quantity: i16,
    pub is_lendable: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::lendings)]
#[diesel(belongs_to(Material, foreign_key=material))]
#[diesel(belongs_to(User, foreign_key=user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Lending {
    pub id: i64,
    pub material: i64,
    pub user_id: Uuid,
    pub quantity: i16,
    pub due_date: Date,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i64,
    pub name: String,
    pub activity: Option<i64>,
    pub kind: MediaKind,
    pub path: String,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct MediaDescription {
    pub name: String,
    pub activity: Option<i64>,
    pub kind: MediaKind,
}

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Identifiable, AsChangeset, Object, Debug,
)]
#[diesel(table_name = crate::schema::members)]
#[diesel(belongs_to(User, foreign_key=user_id))]
#[diesel(belongs_to(Asociation, foreign_key=asociation))]
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
pub struct NaiveMember {
    pub user_id: Uuid,
    pub asociation: Uuid,
    pub is_accepted: bool,
    pub accepted_date: Option<Date>,
    pub expiry_date: Option<Date>,
    pub label: Option<String>,
    pub board_status: BoardStatus,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Debug)]
#[diesel(table_name = crate::schema::organizers)]
#[diesel(belongs_to(Asociation, foreign_key=asociation))]
#[diesel(belongs_to(Activity, foreign_key=activity))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organizer {
    pub id: i64,
    pub asociation: Uuid,
    pub activity: i64,
    pub person_in_charge: Uuid,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::organizers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NaiveOrganizer {
    pub asociation: Uuid,
    pub activity: i64,
    pub person_in_charge: Uuid,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, AsChangeset, Object, Debug)]
#[diesel(table_name = crate::schema::registration)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Registration {
    pub activity: i64,
    pub user_id: Option<Uuid>,
    pub registration_data: Json,
}
