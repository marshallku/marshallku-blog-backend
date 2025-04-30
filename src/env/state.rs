use crate::database::init_db;

use super::app::Env;
use dotenv::dotenv;
use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub host: String,
    pub port: u16,
    pub db: Database,
    pub jwt_secret: String,
    pub cookie_domain: String,
}

impl AppState {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        dotenv().ok();

        let env = Env::new();
        let db = init_db().await?;

        Ok(Self {
            host: env.host.into_owned(),
            port: env.port,
            db,
            jwt_secret: env.jwt_secret.into_owned(),
            cookie_domain: env.cookie_domain.into_owned(),
        })
    }
}
