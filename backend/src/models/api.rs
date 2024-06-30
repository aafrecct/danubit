use crate::models::database as db;

use diesel::prelude::*;
use poem_openapi::{types::multipart, Multipart, Object};
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

// API models

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
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

#[derive(Insertable, Serialize, Deserialize, Object, Debug)]
#[diesel(table_name = crate::schema::media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MediaDescription {
    pub name: String,
    pub activity: Option<i64>,
    pub kind: db::MediaKind,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct MembershipRequest {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct FullMember {
    pub id: i64,
    pub user: db::User,
    pub asociation: Uuid,
    pub is_accepted: bool,
    pub accepted_date: Option<Date>,
    pub expiry_date: Option<Date>,
    pub label: Option<String>,
    pub board_status: db::BoardStatus,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct FullActivity {
    pub activity: db::Activity,
    pub organizers: Vec<db::Asociation>,
    pub people_in_charge: Vec<db::User>,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewFullActivity {
    pub activity: db::NaiveActivity,
    pub organizers: Vec<Uuid>,
    pub people_in_charge: Vec<Uuid>,
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
