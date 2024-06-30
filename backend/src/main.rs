pub mod api;
pub mod auth;
pub mod models;
pub mod schema;
pub mod settings;

use std::env;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection, RunQueryDsl,
};
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let package_version = env::var("CARGO_PKG_VERSION").unwrap_or("dev".to_string());
    let settings = settings::load_settings();
    let server_data = settings::get_server_data(settings);


    tracing_subscriber::fmt::init();

    create_admin_user(&server_data.settings, server_data.data_pool.clone()).ok();
    let url = format!(
        "http://{}:{}",
        &server_data.settings.hostname, &server_data.settings.port
    );
    let api_service = OpenApiService::new(api::DanubitApi, "Danubit", &package_version)
        .server(format!("{}/{}", &url, "api"));
    let auth_service = OpenApiService::new(auth::DanubitAuthApi, "Danubit Auth", &package_version)
        .server(format!("{}/{}", &url, "auth"));

    let docs = api_service.swagger_ui();
    let spec = api_service.spec();
    let auth_docs = auth_service.swagger_ui();
    let auth_spec = auth_service.spec();

    let route = Route::new()
        .nest("/api", api_service)
        .nest("/auth", auth_service)
        .nest("/docs", docs)
        .nest("/docs/auth", auth_docs)
        .at("/docs/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .at("/docs/auth/spec", poem::endpoint::make_sync(move |_| auth_spec.clone()))
        .with(Cors::new())
        .data(server_data);

    Server::new(TcpListener::bind("0.0.0.0:2345"))
        .run(route)
        .await?;
    Ok(())
}

fn create_admin_user(
    settings: &settings::Settings,
    connection_pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<(), String> {
    let password = env::var("ADMIN_PASSWORD").unwrap_or("admin".to_string());
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|x| x.to_string())?
        .to_string();

    let user = models::database::NaiveUser {
        username: settings.admin_username.clone(),
        name: "".to_string(),
        surname: "".to_string(),
        email: "admin@danubit.com".to_string(),
        activated: true,
        password_hash: Some(hashed_password),
        additional_info: None,
    };

    let conn = &mut connection_pool.get().map_err(|x| x.to_string())?;
    diesel::insert_into(schema::users::table)
        .values(user)
        .on_conflict(schema::users::email)
        .do_nothing()
        .execute(conn)
        .map_err(|x| x.to_string())?;

    Ok(())
}
