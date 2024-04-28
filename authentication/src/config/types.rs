
use std::sync::Arc;
use mongodb::Database;
use axum::extract::State;

use super::state::AppContext;

pub type Context = State<Arc<AppContext>>;
pub type DatabaseReference = Arc<Database>;
