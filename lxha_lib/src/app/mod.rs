
pub mod state;

use std::sync::Arc;
use dotenv::dotenv;
use axum::extract::State;

use mongodb::Database;
use self::state::AppContext;

pub type Context = State<Arc<AppContext>>;
pub type DatabaseReference = Arc<Database>;

pub fn env(key: &str) -> String {
    
    dotenv().ok();

    dotenv::var(key)
        .expect(&format!("{} not found in environment variables", key))
}

