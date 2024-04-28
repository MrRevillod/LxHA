
use std::sync::Arc;

use crate::repository::user::UserRespository;

use super::{env, types::DatabaseReference};

use mongodb::{
    Client, Database,
    error::Error as MongoError,
    options::{ClientOptions, ResolverConfig},
};

#[derive(Debug)]
pub struct AppContext {
    pub db: DatabaseReference,
    pub users: UserRespository
}

impl AppContext {
    
    pub fn new(database: Database) -> Arc<AppContext> {

        let arc_db = Arc::new(database);

        let app = AppContext {
            users: UserRespository::new(Arc::clone(&arc_db)),
            db: Arc::clone(&arc_db),
        };

        Arc::new(app)
    }
}

pub async fn database_connection() -> Result<Database, MongoError> {

    let uri = env("DATABASE_URI");
    let db_name = env("DATABASE_NAME");

    let opts = ClientOptions::parse_with_resolver_config(
        &uri, ResolverConfig::cloudflare()).await?
    ;

    let client = Client::with_options(opts)?;

    Ok(client.database(&db_name))
}
