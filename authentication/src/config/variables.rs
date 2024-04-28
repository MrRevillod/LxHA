
use super::env;

use lazy_static::lazy_static;

lazy_static!(

    pub static ref SERVER_ADDR: String = format!("{}:{}", env("AUTH_SERVER_ADDR"), env("AUTH_SERVER_PORT"));
);
