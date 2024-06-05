pub mod models;
pub mod schema;

use self::models::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn main() {
    use self::schema::asociations::dsl::*;
    

    let connection = &mut establish_connection();
    let results = asociations
        .limit(5)
        .select(Asociation::as_select())
        .load(connection)
        .expect("Error loading asociations");

    println!("Displaying {} asociations", results.len());
    for asoc in results {
        println!("{}", asoc.short_name);
        println!("-----------\n");
        println!("Long name: {}", asoc.long_name);
        println!("UUID: {}", asoc.uuid);
    }
}
