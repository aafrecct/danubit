pub mod models;
pub mod schema;
pub mod repo;
pub mod api;

use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use poem::{listener::TcpListener, EndpointExt, Result, Route, Server};
use poem_openapi::OpenApiService;

use dotenvy::dotenv;
use std::env;
// use std::io::{stdin, Read};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}


// pub fn main() {
//     
//     let connection = &mut establish_connection();
//     
//     println!("Add an asociation:\n");
//     let mut json_input = String::new();
//
//     stdin().read_to_string(&mut json_input).unwrap();
//
//     let asoc = repo::create_asociation(connection, &json_input).unwrap();
//     println!("\nSaved asociation {} with id {}", asoc.short_name, asoc.id);
//
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let connection_pool = get_connection_pool();

    let api_service =
        OpenApiService::new(api::DanubitApi, "Danubit", "0.0.1").server("http://localhost:2345");

    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest("/api", api_service)
        .nest("/ui", ui)
        // .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        // .with(Cors::new())
        .data(connection_pool);

    Server::new(TcpListener::bind("0.0.0.0:2345"))
        .run(route)
        .await?;
    Ok(())
}
