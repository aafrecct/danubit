use crate::auth;
use crate::models;
use crate::schema;
use crate::settings::ServerData;
use diesel::prelude::*;
use diesel::{delete, update, BelongingToDsl, SelectableHelper};
use jwt::VerifyWithKey;
use poem::{error, web::Data, Request, Result};
use poem_openapi::{
    auth::Bearer, param::Path, param::Query, payload::Json, OpenApi, SecurityScheme, Tags,
};
use std::vec;
use uuid::Uuid;

pub struct DanubitApi;

#[derive(SecurityScheme)]
#[oai(ty = "bearer", checker = "auth_checker")]
pub struct JWTBearerAuth(auth::AuthScheme);

async fn auth_checker(req: &Request, bearer: Bearer) -> Option<auth::AuthScheme> {
    let server_data = req.data::<ServerData>().unwrap();
    let server_key = &server_data.settings.private_key;
    VerifyWithKey::<auth::AuthScheme>::verify_with_key(bearer.token.as_str(), server_key).ok()
}

#[derive(Tags)]
enum ApiTags {
    Asociations,
    Members,
    Activities,
    Materials,
    Documents,
    Session,
}

#[OpenApi]
impl DanubitApi {
    #[oai(path = "/asociations", method = "get", tag = "ApiTags::Asociations")]
    async fn get_all_asociations(
        &self,
        data: Data<&ServerData>,
    ) -> Result<Json<Vec<models::database::Asociation>>> {
        use schema::asociations::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let result = asociations
            .select(models::database::Asociation::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(path = "/asociations", method = "post", tag = "ApiTags::Asociations")]
    async fn create_asociation(
        &self,
        post_data: Json<models::database::NaiveAsociation>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Asociation>> {
        use schema::asociations::dsl::*;
        auth::check_if_admin(&auth.0, &data.0.settings)?;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let result = diesel::insert_into(asociations)
            .values(post_data.0)
            .returning(models::database::Asociation::as_returning())
            .get_result(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id",
        method = "get",
        tag = "ApiTags::Asociations"
    )]
    async fn get_asociation(
        &self,
        asociation_id: Path<String>,
        data: Data<&ServerData>,
    ) -> Result<Json<models::database::Asociation>> {
        use schema::asociations::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;
        let result = asociations
            .filter(id.eq(uuid))
            .select(models::database::Asociation::as_select())
            .first(conn)
            .map_err(error::NotFound)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/byName/:asociation_short_name",
        method = "get",
        tag = "ApiTags::Asociations"
    )]
    async fn get_asociation_by_name(
        &self,
        asociation_short_name: Path<String>,
        data: Data<&ServerData>,
    ) -> Result<Json<models::database::Asociation>> {
        use schema::asociations::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let result = asociations
            .filter(short_name.eq(asociation_short_name.0))
            .select(models::database::Asociation::as_select())
            .first(conn)
            .map_err(error::NotFound)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id",
        method = "put",
        tag = "ApiTags::Asociations"
    )]
    async fn edit_asociation(
        &self,
        asociation_id: Path<String>,
        update_data: Json<models::database::Asociation>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Asociation>> {
        use schema::asociations::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(&auth.0, models::database::BoardStatus::Board, &uuid)?;

        let result = update(asociations.filter(id.eq(uuid)))
            .set(update_data.0)
            .returning(models::database::Asociation::as_returning())
            .get_result(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(path = "/managers", method = "get", tag = "ApiTags::Asociations")]
    async fn get_all_managers(
        &self,
        data: Data<&ServerData>,
    ) -> Result<Json<Vec<models::database::Manager>>> {
        use schema::managers::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let result = managers
            .select(models::database::Manager::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/membershipRequests",
        method = "get",
        tag = "ApiTags::Members"
    )]
    async fn get_membership_requests(
        &self,
        asociation_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<Vec<models::api::FullMember>>> {
        use schema::members::dsl::*;
        use schema::users;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(&auth.0, models::database::BoardStatus::Board, &uuid)?;

        let requests = members
            .filter(asociation.eq(uuid))
            .filter(is_accepted.eq(false))
            .inner_join(users::table)
            .select((
                models::database::Member::as_select(),
                models::database::User::as_select(),
            ))
            .load(conn)
            .map_err(error::InternalServerError)?;

        let result = requests
            .into_iter()
            .map(|(m, u)| models::api::FullMember {
                id: m.id,
                user: u,
                asociation: m.asociation,
                is_accepted: m.is_accepted,
                accepted_date: m.accepted_date,
                expiry_date: m.expiry_date,
                label: m.label,
                board_status: m.board_status,
            })
            .collect();

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/membershipRequests",
        method = "post",
        tag = "ApiTags::Members"
    )]
    async fn request_membership(
        &self,
        asociation_id: Path<Uuid>,
        post_data: Json<models::api::MembershipRequest>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Member>> {
        use schema::members::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;

        if auth.0.sub != post_data.0.user_id {
            return Err(error::Error::from_string(
                "Cannot request membership for another user.",
                poem::http::StatusCode::BAD_REQUEST,
            ));
        }

        let member = models::database::NaiveMember {
            user_id: post_data.0.user_id,
            asociation: asociation_id.0,
            is_accepted: false,
            accepted_date: None,
            expiry_date: None,
            label: None,
            board_status: models::database::BoardStatus::False,
        };
        let result = diesel::insert_into(members)
            .values(member)
            .returning(models::database::Member::as_returning())
            .get_result(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/membershipRequests/:member_id",
        method = "put",
        tag = "ApiTags::Members"
    )]
    async fn accept_membership(
        &self,
        asociation_id: Path<String>,
        member_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Member>> {
        use schema::members::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let user_uuid = Uuid::try_parse(&member_id.0).map_err(error::BadRequest)?;
        let asociation_uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_if_admin(&auth.0, &data.0.settings).or(auth::check_permissions(
            &auth.0,
            models::database::BoardStatus::Board,
            &asociation_uuid,
        ))?;

        let today = time::OffsetDateTime::now_utc().date();
        let result = update(
            members
                .filter(asociation.eq(asociation_uuid))
                .filter(user_id.eq(user_uuid)),
        )
        .set((
            is_accepted.eq(true),
            accepted_date.eq(today),
            expiry_date.eq(today
                .replace_year(today.year() + 1)
                .map_err(error::InternalServerError)?),
        ))
        .returning(models::database::Member::as_returning())
        .get_result(conn)
        .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/membershipRequests/:member_id",
        method = "delete",
        tag = "ApiTags::Members"
    )]
    async fn deny_membership(
        &self,
        asociation_id: Path<String>,
        member_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<()> {
        use schema::members::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let user_uuid = Uuid::try_parse(&member_id.0).map_err(error::BadRequest)?;
        let asociation_uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(
            &auth.0,
            models::database::BoardStatus::Board,
            &asociation_uuid,
        )?;

        delete(
            members
                .filter(asociation.eq(asociation_uuid))
                .filter(user_id.eq(user_uuid)),
        )
        .execute(conn)
        .map_err(error::InternalServerError)?;

        Ok(())
    }

    #[oai(
        path = "/asociations/:asociation_id/members",
        method = "get",
        tag = "ApiTags::Members"
    )]
    async fn get_members(
        &self,
        asociation_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<Vec<models::api::FullMember>>> {
        use schema::members::dsl::*;
        use schema::users;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(&auth.0, models::database::BoardStatus::Board, &uuid)?;

        let requests = members
            .filter(asociation.eq(uuid))
            .filter(is_accepted.eq(true))
            .inner_join(users::table)
            .select((
                models::database::Member::as_select(),
                models::database::User::as_select(),
            ))
            .load(conn)
            .map_err(error::InternalServerError)?;

        let result = requests
            .into_iter()
            .map(|(m, u)| models::api::FullMember {
                id: m.id,
                user: u,
                asociation: m.asociation,
                is_accepted: m.is_accepted,
                accepted_date: m.accepted_date,
                expiry_date: m.expiry_date,
                label: m.label,
                board_status: m.board_status,
            })
            .collect();

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/members/:member_id",
        method = "put",
        tag = "ApiTags::Members"
    )]
    async fn update_member(
        &self,
        asociation_id: Path<String>,
        member_id: Path<String>,
        update_data: Json<models::database::Member>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Member>> {
        use schema::members::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let user_uuid = Uuid::try_parse(&member_id.0).map_err(error::BadRequest)?;
        let asociation_uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(
            &auth.0,
            models::database::BoardStatus::Board,
            &asociation_uuid,
        )?;

        let result = update(
            members
                .filter(asociation.eq(asociation_uuid))
                .filter(user_id.eq(user_uuid)),
        )
        .set(update_data.0)
        .returning(models::database::Member::as_returning())
        .get_result(conn)
        .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/members/:member_id",
        method = "delete",
        tag = "ApiTags::Members"
    )]
    async fn delete_member(
        &self,
        asociation_id: Path<String>,
        member_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<()> {
        use schema::members::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let user_uuid = Uuid::try_parse(&member_id.0).map_err(error::BadRequest)?;
        let asociation_uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(
            &auth.0,
            models::database::BoardStatus::Board,
            &asociation_uuid,
        )?;

        delete(
            members
                .filter(asociation.eq(asociation_uuid))
                .filter(user_id.eq(user_uuid)),
        )
        .execute(conn)
        .map_err(error::InternalServerError)?;

        Ok(())
    }

    #[oai(
        path = "/asociations/:asociation_id/board",
        method = "get",
        tag = "ApiTags::Members"
    )]
    async fn get_board(
        &self,
        asociation_id: Path<String>,
        data: Data<&ServerData>,
    ) -> Result<Json<Vec<models::database::Member>>> {
        use schema::members::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        let result = members
            .filter(asociation.eq(uuid))
            .filter(is_accepted.eq(true))
            .filter(board_status.eq_any(vec![
                models::database::BoardStatus::Board,
                models::database::BoardStatus::ViceChair,
                models::database::BoardStatus::Chair,
            ]))
            .select(models::database::Member::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/board/:member_id",
        method = "put",
        tag = "ApiTags::Members"
    )]
    async fn add_board_member(
        &self,
        asociation_id: Path<String>,
        member_id: Path<String>,
        update_data: Json<models::database::Member>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Member>> {
        use schema::members::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let user_uuid = Uuid::try_parse(&member_id.0).map_err(error::BadRequest)?;
        let asociation_uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_if_admin(&auth.0, &data.0.settings).or(auth::check_permissions(
            &auth.0,
            models::database::BoardStatus::Chair,
            &asociation_uuid,
        ))?;

        let result = update(
            members
                .filter(asociation.eq(asociation_uuid))
                .filter(user_id.eq(user_uuid)),
        )
        .set(update_data.0)
        .returning(models::database::Member::as_returning())
        .get_result(conn)
        .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/board/:member_id",
        method = "delete",
        tag = "ApiTags::Members"
    )]
    async fn delete_board_member(
        &self,
        asociation_id: Path<String>,
        member_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<()> {
        use schema::members::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let user_uuid = Uuid::try_parse(&member_id.0).map_err(error::BadRequest)?;
        let asociation_uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_if_admin(&auth.0, &data.0.settings).or(auth::check_permissions(
            &auth.0,
            models::database::BoardStatus::Chair,
            &asociation_uuid,
        ))?;

        update(
            members
                .filter(asociation.eq(asociation_uuid))
                .filter(user_id.eq(user_uuid)),
        )
        .set(board_status.eq(models::database::BoardStatus::False))
        .returning(models::database::Member::as_returning())
        .get_result(conn)
        .map_err(error::InternalServerError)?;

        Ok(())
    }

    #[oai(
        path = "/asociations/:asociation_id/publicDocuments",
        method = "get",
        tag = "ApiTags::Documents"
    )]
    async fn list_public_documents(
        &self,
        asociation_id: Path<String>,
        data: Data<&ServerData>,
    ) -> Result<Json<Vec<models::database::Document>>> {
        use schema::documents::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;
        let result = documents
            .filter(asociation.eq(uuid))
            .filter(is_public_accessible.eq(true))
            .select(models::database::Document::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/documents",
        method = "get",
        tag = "ApiTags::Documents"
    )]
    async fn list_documents(
        &self,
        asociation_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<Vec<models::database::Document>>> {
        use schema::documents::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(&auth.0, models::database::BoardStatus::Board, &uuid)?;

        let result = documents
            .filter(asociation.eq(uuid))
            .select(models::database::Document::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/documents",
        method = "post",
        tag = "ApiTags::Documents"
    )]
    async fn create_document(
        &self,
        asociation_id: Path<String>,
        upload: models::api::DocumentUpload,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Document>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/documents/:document_id",
        method = "put",
        tag = "ApiTags::Documents"
    )]
    async fn update_document(
        &self,
        asociation_id: Path<String>,
        document_id: Path<String>,
        update_data: Json<models::api::DocumentDescription>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<Vec<models::database::Document>>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/materials",
        method = "get",
        tag = "ApiTags::Materials"
    )]
    async fn list_all_materials(
        &self,
        asociation_id: Path<String>,
        data: Data<&ServerData>,
    ) -> Result<Json<Vec<models::database::Material>>> {
        use schema::materials::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;
        let result = materials
            .filter(asociation.eq(uuid))
            .select(models::database::Material::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/lendableMaterials",
        method = "get",
        tag = "ApiTags::Materials"
    )]
    async fn list_lendable_materials(
        &self,
        asociation_id: Path<String>,
        data: Data<&ServerData>,
    ) -> Result<Json<Vec<models::database::Material>>> {
        use schema::materials::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;
        let result = materials
            .filter(asociation.eq(uuid))
            .filter(is_lendable.eq(true))
            .select(models::database::Material::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/materials",
        method = "post",
        tag = "ApiTags::Materials"
    )]
    async fn create_material(
        &self,
        asociation_id: Path<String>,
        post_data: Json<models::database::NaiveMaterial>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Material>> {
        use schema::materials::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(&auth.0, models::database::BoardStatus::Board, &uuid)?;

        let result = diesel::insert_into(materials)
            .values(post_data.0)
            .returning(models::database::Material::as_returning())
            .get_result(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/materials/:material_id",
        method = "put",
        tag = "ApiTags::Materials"
    )]
    async fn update_material(
        &self,
        asociation_id: Path<String>,
        material_id: Path<String>,
        update_data: Json<models::database::Material>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Material>> {
        use schema::materials::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(&auth.0, models::database::BoardStatus::Board, &uuid)?;

        let material_id = &material_id.0.parse::<i64>().map_err(error::BadRequest)?;
        let result = update(materials.filter(id.eq(material_id)))
            .set(update_data.0)
            .returning(models::database::Material::as_returning())
            .get_result(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/asociations/:asociation_id/materials/:material_id",
        method = "delete",
        tag = "ApiTags::Materials"
    )]
    async fn delete_material(
        &self,
        asociation_id: Path<String>,
        material_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<()> {
        use schema::materials::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;

        auth::check_permissions(&auth.0, models::database::BoardStatus::Board, &uuid)?;

        let material_id = &material_id.0.parse::<i64>().map_err(error::BadRequest)?;
        delete(materials.filter(id.eq(material_id)))
            .execute(conn)
            .map_err(error::InternalServerError)?;

        Ok(())
    }

    #[oai(
        path = "/publicActivities",
        method = "get",
        tag = "ApiTags::Activities"
    )]
    async fn list_public_activities(
        &self,
        asociation_filter: Query<Option<Uuid>>,
        data: Data<&ServerData>,
    ) -> Result<Json<Vec<models::api::FullActivity>>> {
        use schema::activities;
        use schema::asociations;
        use schema::organizers;
        use schema::users;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let public_activities = activities::table
            .filter(activities::access.eq(models::database::ActivityAccess::Public))
            .select(models::database::Activity::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        let mut activity_organizers = models::database::Organizer::belonging_to(&public_activities)
            .inner_join(asociations::table)
            .inner_join(users::table)
            .into_boxed();

        if asociation_filter.0.is_some() {
            activity_organizers =
                activity_organizers.filter(organizers::asociation.eq(asociation_filter.0.unwrap()));
        };

        let activity_organizers: Vec<(
            models::database::Organizer,
            models::database::Asociation,
            models::database::User,
        )> = activity_organizers
            .select((
                models::database::Organizer::as_select(),
                models::database::Asociation::as_select(),
                models::database::User::as_select(),
            ))
            .load(conn)
            .map_err(error::InternalServerError)?;

        let result: Vec<models::api::FullActivity> = activity_organizers
            .grouped_by(&public_activities)
            .into_iter()
            .zip(public_activities)
            .map(|(org, act)| {
                let (asocs, people) = org.into_iter().map(|(_, a, b)| (a, b)).unzip();
                models::api::FullActivity {
                    activity: act,
                    organizers: asocs,
                    people_in_charge: people,
                }
            })
            .filter(|a| !a.organizers.is_empty())
            .collect();

        Ok(Json(result))
    }

    #[oai(
        path = "/memberActivities",
        method = "get",
        tag = "ApiTags::Activities"
    )]
    async fn list_member_activities(
        &self,
        asociation_filter: Query<Option<Uuid>>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<Vec<models::api::FullActivity>>> {
        use schema::activities;
        use schema::asociations;
        use schema::organizers;
        use schema::users;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let member_activities = activities::table
            .filter(activities::access.eq(models::database::ActivityAccess::Members))
            .select(models::database::Activity::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        let mut activity_organizers = models::database::Organizer::belonging_to(&member_activities)
            .inner_join(asociations::table)
            .inner_join(users::table)
            .filter(organizers::asociation.eq_any(auth.0.member_of))
            .into_boxed();

        if asociation_filter.0.is_some() {
            activity_organizers =
                activity_organizers.filter(organizers::asociation.eq(asociation_filter.0.unwrap()));
        };

        let activity_organizers: Vec<(
            models::database::Organizer,
            models::database::Asociation,
            models::database::User,
        )> = activity_organizers
            .select((
                models::database::Organizer::as_select(),
                models::database::Asociation::as_select(),
                models::database::User::as_select(),
            ))
            .load(conn)
            .map_err(error::InternalServerError)?;

        let result: Vec<models::api::FullActivity> = activity_organizers
            .grouped_by(&member_activities)
            .into_iter()
            .zip(member_activities)
            .map(|(org, act)| {
                let (asocs, people) = org.into_iter().map(|(_, a, b)| (a, b)).unzip();
                models::api::FullActivity {
                    activity: act,
                    organizers: asocs,
                    people_in_charge: people,
                }
            })
            .collect();

        Ok(Json(result))
    }

    #[oai(path = "/boardActivities", method = "get", tag = "ApiTags::Activities")]
    async fn list_board_activities(
        &self,
        asociation_filter: Query<Option<Uuid>>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<Vec<models::api::FullActivity>>> {
        use schema::activities;
        use schema::asociations;
        use schema::organizers;
        use schema::users;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let board_activities = activities::table
            .filter(activities::access.eq(models::database::ActivityAccess::Board))
            .select(models::database::Activity::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        let mut activity_organizers = models::database::Organizer::belonging_to(&board_activities)
            .inner_join(asociations::table)
            .inner_join(users::table)
            .filter(organizers::asociation.eq_any(auth.0.board_of))
            .into_boxed();

        if asociation_filter.0.is_some() {
            activity_organizers =
                activity_organizers.filter(organizers::asociation.eq(asociation_filter.0.unwrap()));
        };

        let activity_organizers: Vec<(
            models::database::Organizer,
            models::database::Asociation,
            models::database::User,
        )> = activity_organizers
            .select((
                models::database::Organizer::as_select(),
                models::database::Asociation::as_select(),
                models::database::User::as_select(),
            ))
            .load(conn)
            .map_err(error::InternalServerError)?;

        let result: Vec<models::api::FullActivity> = activity_organizers
            .grouped_by(&board_activities)
            .into_iter()
            .zip(board_activities)
            .map(|(org, act)| {
                let (asocs, people) = org.into_iter().map(|(_, a, b)| (a, b)).unzip();
                models::api::FullActivity {
                    activity: act,
                    organizers: asocs,
                    people_in_charge: people,
                }
            })
            .collect();

        Ok(Json(result))
    }

    #[oai(path = "/activities", method = "post", tag = "ApiTags::Activities")]
    async fn create_activity(
        &self,
        post_data: Json<models::api::NewFullActivity>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Activity>> {
        use schema::activities::dsl::*;
        use schema::organizers;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;

        auth::check_if_admin(&auth.0, &data.settings).or(auth::check_permissions_in_any(
            &auth.0,
            models::database::BoardStatus::Board,
            &post_data.0.organizers,
        ))?;

        let new_activity = diesel::insert_into(activities)
            .values(post_data.0.activity)
            .returning(models::database::Activity::as_returning())
            .get_result(conn)
            .map_err(error::InternalServerError)?;

        let organizers = post_data
            .0
            .organizers
            .into_iter()
            .zip(post_data.0.people_in_charge);

        diesel::insert_into(organizers::table)
            .values(
                organizers
                    .map(|(asoc, person)| models::database::NaiveOrganizer {
                        asociation: asoc,
                        activity: new_activity.id,
                        person_in_charge: person,
                    })
                    .collect::<Vec<models::database::NaiveOrganizer>>(),
            )
            .execute(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(new_activity))
    }

    #[oai(
        path = "/activities/:activity_id",
        method = "get",
        tag = "ApiTags::Activities"
    )]
    async fn get_activity(
        &self,
        activity_id: Path<i64>,
        data: Data<&ServerData>,
    ) -> Result<Json<models::api::FullActivity>> {
        use schema::activities;
        use schema::asociations;
        use schema::users;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let activity = activities::table
            .filter(activities::id.eq(activity_id.0))
            .select(models::database::Activity::as_select())
            .first(conn)
            .map_err(error::InternalServerError)?;

        let (asocs, people) = models::database::Organizer::belonging_to(&activity)
            .inner_join(asociations::table)
            .inner_join(users::table)
            .select((
                models::database::Organizer::as_select(),
                models::database::Asociation::as_select(),
                models::database::User::as_select(),
            ))
            .load(conn)
            .map_err(error::InternalServerError)?
            .into_iter()
            .map(|(_, a, b)| (a, b))
            .unzip();

        let result = models::api::FullActivity {
            activity,
            organizers: asocs,
            people_in_charge: people,
        };

        Ok(Json(result))
    }

    #[oai(
        path = "/activities/:activity_id",
        method = "put",
        tag = "ApiTags::Activities"
    )]
    async fn put_activity(
        &self,
        activity_id: Path<i64>,
        update_data: Json<models::api::NewFullActivity>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Activity>> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id",
        method = "delete",
        tag = "ApiTags::Activities"
    )]
    async fn delete_activity(
        &self,
        activity_id: Path<i64>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<()> {
        use schema::activities::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;

        auth::check_permissions_in_any(
            &auth.0,
            models::database::BoardStatus::Board,
            &auth.0.board_of,
        )?;

        delete(activities.filter(id.eq(&activity_id.0)))
            .execute(conn)
            .map_err(error::InternalServerError)?;

        Ok(())
    }

    #[oai(
        path = "/activities/:activity_id/media",
        method = "post",
        tag = "ApiTags::Activities"
    )]
    async fn add_activity_media(
        &self,
        asociation_id: Path<String>,
        upload: models::api::MediaUpload,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Activity>> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id/media/:media_id",
        method = "delete",
        tag = "ApiTags::Activities"
    )]
    async fn delete_activity_media(
        &self,
        asociation_id: Path<String>,
        media_id: Path<String>,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<models::database::Activity>> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id/registration",
        method = "get",
        tag = "ApiTags::Activities"
    )]
    async fn list_activity_registry(
        &self,
        activity_id: Path<i64>,
        data: Data<&ServerData>,
    ) -> Result<Json<Vec<models::database::Registration>>> {
        use schema::registration::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let result = registration
            .filter(activity.eq(&activity_id.0))
            .select(models::database::Registration::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }
    #[oai(
        path = "/activities/:activity_id/registration",
        method = "post",
        tag = "ApiTags::Activities"
    )]
    async fn register_for_activity(
        &self,
        activity_id: Path<i64>,
        post_data: Json<models::database::Registration>,
        data: Data<&ServerData>,
    ) -> Result<Json<models::database::Registration>> {
        use schema::registration::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;

        let result = diesel::insert_into(registration)
            .values(post_data.0)
            .returning(models::database::Registration::as_returning())
            .get_result(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(
        path = "/activities/:activity_id/registration",
        method = "delete",
        tag = "ApiTags::Activities"
    )]
    async fn unregister_from_activity(
        &self,
        activity_id: Path<String>,
        data: Data<&ServerData>,
    ) -> Result<Json<models::database::Activity>> {
        todo!()
    }

    #[oai(path = "/session/board_of", method = "get", tag = "ApiTags::Session")]
    async fn get_session_asociations_board(
        &self,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<Vec<models::database::Asociation>>> {
        use schema::asociations::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let result = asociations
            .filter(id.eq_any(auth.0.board_of))
            .select(models::database::Asociation::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(path = "/session/member_of", method = "get", tag = "ApiTags::Session")]
    async fn get_session_asociations_member(
        &self,
        data: Data<&ServerData>,
        auth: JWTBearerAuth,
    ) -> Result<Json<Vec<models::database::Asociation>>> {
        use schema::asociations::dsl::*;

        let conn = &mut data.data_pool.get().map_err(error::InternalServerError)?;
        let result = asociations
            .filter(id.eq_any(auth.0.member_of))
            .select(models::database::Asociation::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }
}
