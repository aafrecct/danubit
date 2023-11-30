use poem_openapi::{payload::Json, OpenApi, Tags};
use poem::{Result, web::Data};
use diesel::{PgConnection, SelectableHelper};
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
use crate::models;
use crate::schema;
pub struct DanubitApi;

#[derive(Tags)]
enum ApiTags {
    Asociations,
    Members,
    Activities,
    Materials,
    Documents,
    Media
}

#[OpenApi]
impl DanubitApi {

    #[oai(path = "/asociations", method = "get", tag = "ApiTags::Asociations")]
    async fn get_all_asociations(&self, pool: Data<&Pool<ConnectionManager<PgConnection>>>) -> Result<Json<Vec<models::Asociation>>> {
        use schema::asociations::dsl::*;

        let conn = &mut pool.get().unwrap();
        let result = asociations
            .select(models::Asociation::as_select())
            .load(conn)
            .expect("Whoops");

        Ok(Json(result))
    }

}
