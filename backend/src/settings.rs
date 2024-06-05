use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

#[derive(Clone)]
pub struct ServerData {
    pub data_pool: Pool<ConnectionManager<PgConnection>>,
    pub settings: Settings,
}

#[derive(Clone)]
pub struct Settings {
    pub hostname: String,
    pub port: String,
    pub debug: bool,
    pub database_url: String,
    pub private_key: Hmac<Sha256>,
    pub admin_username: String,
}

pub fn load_settings() -> Settings {
    dotenv().ok();
    let private_key = Hmac::<Sha256>::new_from_slice(
        env::var("PRIVATE_KEY")
            .unwrap_or("VERY_BAD_ONLY_FOR_DEV_KEY".to_string())
            .as_bytes(),
    )
    .expect("Critical: Invalid Private Key!");
    Settings {
        hostname: env::var("HOSTNAME").unwrap_or("localhost".to_string()),
        port: env::var("PORT").unwrap_or("2345".to_string()),
        debug: env::var("DEBUG").unwrap_or("false".to_string()) == "true",
        database_url: env::var("DATABASE_URL")
            .unwrap_or("postgres://danubit:danubit@localhost:5432/danubit".to_string()),
        private_key,
        admin_username: env::var("ADMIN_USERNAME").unwrap_or("admin".to_string()),
    }
}

pub fn get_connection_pool(database_url: &String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn get_server_data(settings: Settings) -> ServerData {
    ServerData {
        data_pool: get_connection_pool(&settings.database_url),
        settings,
    }
}
