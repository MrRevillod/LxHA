
pub mod variables;
pub mod state;
pub mod types;

use dotenv::dotenv;

pub fn env(key: &str) -> String {
    dotenv().ok();

    dotenv::var(key)
        .expect(&format!("{} not found in environment variables", key))
}
