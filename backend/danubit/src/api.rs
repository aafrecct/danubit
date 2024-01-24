use crate::models;
use crate::schema;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{update, PgConnection, SelectableHelper};
use poem::{error, web::Data, Result};
use poem_openapi::{param::Path, param::Query, payload::Json, OpenApi, Tags};
use tracing::event;
use uuid::Uuid;

pub struct DanubitApi;

#[derive(Tags)]
enum ApiTags {
    Asociations,
    Members,
    Activities,
    Materials,
    Documents,
}

#[OpenApi]
impl DanubitApi {
    #[oai(path = "/asociations", method = "get", tag = "ApiTags::Asociations")]
    async fn get_all_asociations(
        &self,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Asociation>>> {
        use schema::asociations::dsl::*;

        let conn = &mut pool.get().map_err(error::InternalServerError)?;
        let result = asociations
            .select(models::Asociation::as_select())
            .load(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(path = "/asociations", method = "post", tag = "ApiTags::Asociations")]
    async fn create_asociation(
        &self,
        post_data: Json<models::NewAsociation>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Asociation>> {
        use schema::asociations::dsl::*;

        let conn = &mut pool.get().map_err(error::InternalServerError)?;
        let result = diesel::insert_into(asociations)
            .values(post_data.0)
            .returning(models::Asociation::as_returning())
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
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Asociation>> {
        use schema::asociations::dsl::*;

        let conn = &mut pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;
        let result = asociations
            .filter(id.eq(uuid))
            .select(models::Asociation::as_select())
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
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Asociation>> {
        use schema::asociations::dsl::*;

        let conn = &mut pool.get().map_err(error::InternalServerError)?;
        let result = asociations
            .filter(short_name.eq(asociation_short_name.0))
            .select(models::Asociation::as_select())
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
        update_data: Json<models::Asociation>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Asociation>> {
        use schema::asociations::dsl::*;

        let conn = &mut pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;
        let result = update(asociations.filter(id.eq(uuid)))
            .set(update_data.0)
            .returning(models::Asociation::as_returning())
            .get_result(conn)
            .map_err(error::InternalServerError)?;

        Ok(Json(result))
    }

    #[oai(path = "/managers", method = "get", tag = "ApiTags::Asociations")]
    async fn get_all_managers(
        &self,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Manager>>> {
        use schema::managers::dsl::*;

        let conn = &mut pool.get().map_err(error::InternalServerError)?;
        let result = managers
            .select(models::Manager::as_select())
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
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::MembershipRequest>>> {
        use schema::members::dsl::*;

        let conn = &mut pool.get().map_err(error::InternalServerError)?;
        let uuid = Uuid::try_parse(&asociation_id.0).map_err(error::BadRequest)?;
        let requests = members
            .filter(asociation.eq(uuid))
            .filter(is_accepted.eq(false))
            .select((user_id, asociation, is_accepted))
            .load::<(Uuid, Uuid, bool)>(conn)
            .map_err(error::InternalServerError)?;

        let result = requests
            .into_iter()
            .map(|r| models::MembershipRequest {
                user_id: r.0,
                asociation: r.1,
                is_accepted: r.2,
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
        asociation_id: Path<String>,
        post_data: Json<models::NewMember>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::MembershipRequest>>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/membershipRequests/:member_id",
        method = "patch",
        tag = "ApiTags::Members"
    )]
    async fn accept_membership(
        &self,
        asociation_id: Path<String>,
        member_id: Path<String>,
        patch_data: Json<models::AcceptAction>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Member>> {
        todo!()
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
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::MembershipRequest>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/members",
        method = "get",
        tag = "ApiTags::Members"
    )]
    async fn get_members(
        &self,
        asociation_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Member>>> {
        todo!()
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
        update_data: Json<models::Member>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Member>> {
        todo!()
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
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Member>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/publicDocuments",
        method = "get",
        tag = "ApiTags::Documents"
    )]
    async fn list_public_documents(
        &self,
        asociation_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Document>>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/documents",
        method = "get",
        tag = "ApiTags::Documents"
    )]
    async fn list_documents(
        &self,
        asociation_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Document>>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/documents",
        method = "post",
        tag = "ApiTags::Documents"
    )]
    async fn create_document(
        &self,
        asociation_id: Path<String>,
        upload: models::DocumentUpload,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Document>>> {
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
        update_data: Json<models::DocumentDescription>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Document>>> {
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
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Material>>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/lendableMaterials",
        method = "get",
        tag = "ApiTags::Materials"
    )]
    async fn list_lendable_materials(
        &self,
        asociation_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Material>>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/materials",
        method = "post",
        tag = "ApiTags::Materials"
    )]
    async fn create_material(
        &self,
        asociation_id: Path<String>,
        post_data: Json<models::NewMaterial>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Material>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/materials/:material_id",
        method = "put",
        tag = "ApiTags::Materials"
    )]
    async fn update_material(
        &self,
        asociation_id: Path<String>,
        update_data: Json<models::Material>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Material>> {
        todo!()
    }

    #[oai(
        path = "/asociations/:asociation_id/materials/:material_id",
        method = "delete",
        tag = "ApiTags::Materials"
    )]
    async fn delete_material(
        &self,
        asociation_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Material>> {
        todo!()
    }

    #[oai(
        path = "/publicActivities",
        method = "get",
        tag = "ApiTags::Activities"
    )]
    async fn list_public_activities(
        &self,
        asociation_filter: Query<Vec<Uuid>>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Activity>>> {
        todo!()
    }

    #[oai(
        path = "/memberActivities",
        method = "get",
        tag = "ApiTags::Activities"
    )]
    async fn list_member_activities(
        &self,
        asociation_filter: Query<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Activity>>> {
        todo!()
    }

    #[oai(path = "/boardActivities", method = "get", tag = "ApiTags::Activities")]
    async fn list_board_activities(
        &self,
        asociation_filter: Query<Vec<Uuid>>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<Vec<models::Activity>>> {
        todo!()
    }

    #[oai(path = "/activities", method = "post", tag = "ApiTags::Activities")]
    async fn create_activity(
        &self,
        post_data: Json<models::NewActivity>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Activity>> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id",
        method = "get",
        tag = "ApiTags::Activities"
    )]
    async fn get_activity(
        &self,
        asociation_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Activity>> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id",
        method = "put",
        tag = "ApiTags::Activities"
    )]
    async fn put_activity(
        &self,
        asociation_id: Path<String>,
        update_data: Json<models::Activity>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Activity>> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id",
        method = "delete",
        tag = "ApiTags::Activities"
    )]
    async fn delete_activity(
        &self,
        activity_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Activity>> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id/media",
        method = "post",
        tag = "ApiTags::Activities"
    )]
    async fn add_activity_media(
        &self,
        asociation_id: Path<String>,
        upload: models::MediaUpload,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Activity>> {
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
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Activity>> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id/registration",
        method = "post",
        tag = "ApiTags::Activities"
    )]
    async fn register_for_activity(
        &self,
        asociation_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<()> {
        todo!()
    }

    #[oai(
        path = "/activities/:activity_id/registration",
        method = "delete",
        tag = "ApiTags::Activities"
    )]
    async fn unregister_from_activity(
        &self,
        activity_id: Path<String>,
        pool: Data<&Pool<ConnectionManager<PgConnection>>>,
    ) -> Result<Json<models::Activity>> {
        todo!()
    }
}
