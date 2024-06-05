use crate::models;
use crate::schema;
use crate::settings::ServerData;
use crate::settings::Settings;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use jwt::SignWithKey;
use poem::http::StatusCode;
use poem::{error, web::Data, Result};
use poem_openapi::payload::PlainText;
use poem_openapi::{payload::Json, Object, OpenApi};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthScheme {
    pub sub: Uuid,
    pub iat: u64,
    pub exp: u64,
    pub username: String,
    pub manager_of: Vec<Uuid>,
    pub chair_of: Vec<Uuid>,
    pub board_of: Vec<Uuid>,
    pub member_of: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct LoginResponse {
    pub id: Uuid,
    pub username: String,
    pub token: String,
    pub expires_at: u64,
    pub manager_of: Vec<Uuid>,
    pub chair_of: Vec<Uuid>,
    pub board_of: Vec<Uuid>,
    pub member_of: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct UserSignup {
    username: String,
    name: String,
    surname: String,
    email: String,
    password: String,
    additional_info: Option<JsonValue>,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Standin {
    username: String,
    name: String,
    surname: String,
    email: String,
}

pub struct DanubitAuthApi;

#[OpenApi]
impl DanubitAuthApi {
    #[oai(path = "/login", method = "post")]
    async fn login(
        &self,
        data: Data<&ServerData>,
        post_data: Json<Login>,
    ) -> Result<Json<LoginResponse>> {
        use schema::asociations;
        use schema::managers;
        use schema::members;
        use schema::users::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let user = users
            .filter(email.eq(post_data.0.email))
            .select(models::User::as_select())
            .first(conn)
            .map_err(error::InternalServerError)?;

        let saved_password = user.password_hash.ok_or(error::NotFoundError)?;
        let parsed_hash = PasswordHash::new(&saved_password)
            .map_err(|e| argon_error_to_api_error(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Argon2::default()
            .verify_password(post_data.0.password.as_bytes(), &parsed_hash)
            .map_err(|e| argon_error_to_api_error(e, StatusCode::FORBIDDEN))?;

        let manager_id = managers::table
            .filter(managers::user_id.eq(&user.id))
            .select(managers::id)
            .first::<i64>(conn)
            .optional()
            .map_err(error::InternalServerError)?;

        let manager_of = match manager_id {
            None => Vec::<Uuid>::new(),
            Some(x) => asociations::table
                .filter(asociations::manager.eq(x))
                .select(asociations::id)
                .load::<Uuid>(conn)
                .map_err(error::InternalServerError)?,
        };

        let chair_of = members::table
            .filter(members::user_id.eq(&user.id))
            .filter(members::board_status.eq(models::BoardStatus::Chair))
            .select(members::asociation)
            .load::<Uuid>(conn)
            .map_err(error::InternalServerError)?;

        let board_of = members::table
            .filter(members::user_id.eq(&user.id))
            .filter(members::board_status.eq_any(vec![
                models::BoardStatus::Board,
                models::BoardStatus::ViceChair,
                models::BoardStatus::Chair,
            ]))
            .select(members::asociation)
            .load::<Uuid>(conn)
            .map_err(error::InternalServerError)?;

        let member_of = members::table
            .filter(members::user_id.eq(&user.id))
            .select(members::asociation)
            .load::<Uuid>(conn)
            .map_err(error::InternalServerError)?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(error::InternalServerError)?;

        let auth_scheme = AuthScheme {
            sub: user.id,
            iat: timestamp.as_secs(),
            exp: timestamp.as_secs() + 60 * 60 * 24,
            username: user.username.clone(),
            manager_of: manager_of.clone(),
            chair_of: chair_of.clone(),
            board_of: board_of.clone(),
            member_of: member_of.clone(),
        }
        .sign_with_key(&data.0.settings.private_key)
        .map_err(error::InternalServerError)?;

        let response = LoginResponse {
            id: user.id,
            username: user.username,
            token: auth_scheme,
            expires_at: timestamp.as_secs() + 60 * 60 * 24,
            manager_of,
            chair_of,
            board_of,
            member_of,
        };

        Ok(Json(response))
    }

    #[oai(path = "/signup", method = "post")]
    async fn signup(&self, data: Data<&ServerData>, post_data: Json<UserSignup>) -> Result<()> {
        use schema::users::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(post_data.0.password.as_bytes(), &salt)
            .map_err(|e| argon_error_to_api_error(e, StatusCode::INTERNAL_SERVER_ERROR))?
            .to_string();

        let user = models::NewUser {
            username: post_data.0.username,
            name: post_data.0.name,
            surname: post_data.0.surname,
            email: post_data.0.email,
            activated: true,
            password_hash: Some(hashed_password),
            additional_info: post_data.0.additional_info,
        };

        diesel::insert_into(users)
            .values(user)
            .execute(conn)
            .map_err(error::InternalServerError)?;

        Ok(())
    }

    #[oai(path = "/standin", method = "post")]
    async fn standin(&self, data: Data<&ServerData>, post_data: Json<Standin>) -> Result<()> {
        use schema::users::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;

        let user = models::NewUser {
            username: post_data.0.username,
            name: post_data.0.name,
            surname: post_data.0.surname,
            email: post_data.0.email,
            activated: true,
            password_hash: None,
            additional_info: None,
        };

        diesel::insert_into(users)
            .values(user)
            .execute(conn)
            .map_err(error::InternalServerError)?;

        Ok(())
    }

    #[oai(path = "/logout", method = "post")]
    async fn logout(
        &self,
        data: Data<&ServerData>,
        post_data: Json<Login>,
    ) -> Result<PlainText<String>> {
        todo!()
    }

    #[oai(path = "/change_username", method = "post")]
    async fn change_username(
        &self,
        data: Data<&ServerData>,
        post_data: Json<Login>,
    ) -> Result<PlainText<String>> {
        todo!()
    }

    #[oai(path = "/change_password", method = "post")]
    async fn change_password(
        &self,
        data: Data<&ServerData>,
        post_data: Json<Login>,
    ) -> Result<PlainText<String>> {
        todo!()
    }
}

fn argon_error_to_api_error(argon_error: impl ToString, status_code: StatusCode) -> error::Error {
    let error_string = argon_error.to_string();
    error::Error::from_string(error_string, status_code)
}

pub fn check_permissions(
    auth_scheme: &AuthScheme,
    min_needed_permision: models::BoardStatus,
    asociation: &Uuid,
) -> Result<(), error::Error> {
    if (min_needed_permision <= models::BoardStatus::False
        && auth_scheme.member_of.contains(asociation))
        || (auth_scheme.board_of.contains(asociation)
            && min_needed_permision <= models::BoardStatus::Board)
        || (auth_scheme.chair_of.contains(asociation)
            && min_needed_permision <= models::BoardStatus::Chair)
    {
        return Ok(());
    };
    Err(error::Error::from_string(
        "Insufficient permissions",
        StatusCode::FORBIDDEN,
    ))
}

pub fn check_if_manager(auth_scheme: &AuthScheme, asociation: &Uuid) -> Result<(), error::Error> {
    if auth_scheme.manager_of.contains(asociation) {
        return Ok(());
    }
    Err(error::Error::from_string(
        "Not a manager of this asociation",
        StatusCode::FORBIDDEN,
    ))
}

pub fn check_if_admin(auth_scheme: &AuthScheme, settings: &Settings) -> Result<(), error::Error> {
    if auth_scheme.username == settings.admin_username {
        return Ok(());
    }
    Err(error::Error::from_string(
        "Missing admin priviledges",
        StatusCode::FORBIDDEN,
    ))
}

pub fn check_permissions_in_any(
    auth_scheme: &AuthScheme,
    min_needed_permision: models::BoardStatus,
    asociations: &Vec<Uuid>,
) -> Result<(), error::Error> {
    for uuid in asociations {
        if (min_needed_permision <= models::BoardStatus::False
            && auth_scheme.member_of.contains(uuid))
            || (auth_scheme.board_of.contains(uuid)
                && min_needed_permision <= models::BoardStatus::Board)
            || (auth_scheme.chair_of.contains(uuid)
                && min_needed_permision <= models::BoardStatus::Chair)
        {
            return Ok(());
        };
    }
    Err(error::Error::from_string(
        "Insufficient permissions",
        StatusCode::FORBIDDEN,
    ))
}
