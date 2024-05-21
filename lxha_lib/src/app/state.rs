
use std::sync::Arc;
use super::{constants::{DB_NAME, DB_URI}, DatabaseReference};

use crate::repository::{
    token::TokenRepository,
    user::UserRespository,
    instance::InstanceRepository,
};

use mongodb::{
    Client, Database,
    error::Error as MongoError,
    options::{ClientOptions, ResolverConfig},
};

#[derive(Debug)]
pub struct AppContext {
    pub db: DatabaseReference,
    pub users: UserRespository,
    pub tokens: TokenRepository,
    pub instances: InstanceRepository
}

impl AppContext {
    
    pub fn new(database: Database) -> Arc<AppContext> {

        let arc_db = Arc::new(database);

        let app = AppContext {
            users: UserRespository::new(Arc::clone(&arc_db)),
            instances: InstanceRepository::new(Arc::clone(&arc_db)),
            tokens: TokenRepository::new(Arc::clone(&arc_db)),
            db: Arc::clone(&arc_db),
        };

        Arc::new(app)
    }
}

pub async fn database_connection() -> Result<Database, MongoError> {

    let opts = ClientOptions::parse_with_resolver_config(
        &DB_URI.to_string(), ResolverConfig::cloudflare()).await?
    ;

    let client = Client::with_options(opts)?;

    Ok(client.database(&DB_NAME))
}
