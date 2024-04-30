
pub mod variables;

use dotenv::dotenv;

pub fn env(key: &str) -> String {
    dotenv().ok();

    dotenv::var(key)
        .expect(&format!("{} not found in environment variables", key))
}
