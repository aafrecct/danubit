use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::error::Error;
use crate::models;
use crate::schema;


pub fn create_asociation(conn: &mut PgConnection, asoc: &str) 
    -> Result<models::Asociation, Box<dyn Error>> {
    use models::NewAsociation;

    let new_asoc: NewAsociation = serde_json::from_str(asoc)?;  

    Ok(diesel::insert_into(schema::asociations::table)
        .values(&new_asoc)
        .returning(models::Asociation::as_returning())
        .get_result(conn)?)
}
