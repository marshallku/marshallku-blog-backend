use crate::database::init_db;

use super::app::Env;
use dotenv::dotenv;
use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub host: String,
    pub port: u16,
    pub db: Database,
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
        })
    }
}
