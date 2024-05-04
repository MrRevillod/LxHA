
use super::env;
use lazy_static::lazy_static;

lazy_static!(

    // Database env variables

    pub static ref DB_NAME: String = env("DB_NAME");
    pub static ref DB_URI: String = env("DB_URI");

    // Microservices env variables

    pub static ref AUTH_SERVICE_ADDR: String = env("AUTH_SERVICE_ADDR");
    pub static ref CLIENT_SERVICE_ADDR: String = env("CLIENT_SERVICE_ADDR");

    // Other env variables

    pub static ref JWT_SECRET: String = env("JWT_SECRET");
);
