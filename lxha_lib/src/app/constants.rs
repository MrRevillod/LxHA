
use std::collections::HashMap;

use super::env;
use lazy_static::lazy_static;

lazy_static!(

    // Database env variables

    pub static ref DB_NAME: String = env("DB_NAME");
    pub static ref DB_URI: String = env("DB_PROD_URI");

    // Microservices env variables

    // - Axum rs addr kind microservices (0.0.0.0:PORT)

    pub static ref AUTH_SERVICE_ADDR: String = env("AUTH_SERVICE_ADDR");
    pub static ref DASHBOARD_SERVICE_ADDR: String = env("DASHBOARD_SERVICE_ADDR");
    pub static ref MAILER_SERVICE_ADDR: String = env("MAILER_SERVICE_ADDR");

    // - Microservices complete url for http requests

    pub static ref FRONTEND_SERVICE_URL: String = env("FRONTEND_SERVICE_URL");
    pub static ref MAILER_SERVICE_URL: String = env("MAILER_SERVICE_URL");
    pub static ref AUTH_SERVICE_URL: String = env("AUTH_SERVICE_URL");
    pub static ref DASHBOARD_SERVICE_URL: String = env("DASHBOARD_SERVICE_URL");

    // Other env variables

    pub static ref JWT_SECRET: String = env("JWT_SECRET");
    pub static ref INCUS_API: String = env("INCUS_API");
    
    pub static ref CRT_PATH: String = env("CRT_PATH");
    pub static ref CRT_KEY_PATH: String = env("CRT_KEY_PATH");

    pub static ref LOCAL_NET_ADDR: String = env("LOCAL_NET_ADDR");
    pub static ref LOCAL_NET_MASK: String = env("LOCAL_NET_MASK");

    pub static ref DEFAULT_USER_PASSWORD: String = env("DEFAULT_USER_PASSWORD");
    pub static ref LXHA_MAIL_ADRESS: String = env("MAILER_SERVICE_MAIL_ADRESS");
    pub static ref LXHA_MAIL_HOST: String = env("MAILER_SERVICE_MAIL_HOST");
    pub static ref LXHA_MAIL_PASSWORD: String = env("MAILER_SERVICE_MAIL_PASSWORD");

    pub static ref SERVICES: HashMap<&'static str, String> = HashMap::from([
        ("AUTH", AUTH_SERVICE_URL.to_string()),
        ("DASHBOARD", DASHBOARD_SERVICE_URL.to_string()),
        ("MAILER", MAILER_SERVICE_URL.to_string()),
    ]);
);
