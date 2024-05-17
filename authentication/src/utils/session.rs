
use cookie::{Cookie, SameSite};
use chrono::{Utc, Duration as ChronoDuration};
use axum_responses::{AxumResult, HttpResponse};

use jsonwebtoken::*;

use lxha_lib::models::user::User;
use lxha_lib::app::constants::JWT_SECRET;

use crate::models::JwtPayload;

pub fn sign_jwt(payload: JwtPayload, secret: &String) -> AxumResult<String> {

    let token = encode(&Header::default(), &payload,
        &EncodingKey::from_secret(secret.as_bytes())
    );

    if let Err(_) = token {
        return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    Ok(token.unwrap())
}

pub fn decode_jwt(token: &String, secret: &String) -> AxumResult<JwtPayload> {

    let payload = decode::<JwtPayload>(token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default()
    );

    if let Err(_) = payload {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    let payload = payload.unwrap().claims;

    let exp = payload.exp as i64;
    let now = Utc::now().timestamp();

    if exp < now {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    Ok(payload)
}

/// Returns a new session token with the session/user id of the request 

pub fn new_token_from_refresh(refresh_token: &String) -> AxumResult<(String, String)> {

    let payload = decode_jwt(refresh_token, &JWT_SECRET)?;
    let exp = (Utc::now() + ChronoDuration::minutes(60)).timestamp() as usize;

    let new_payload = JwtPayload { 
        id: payload.id.clone(), 
        email: payload.email, 
        exp
    };

    let new_token = sign_jwt(new_payload, &JWT_SECRET)?;

    Ok((new_token, payload.id))
}

pub fn new_token(kind: &str, user: &User, key: &String) -> AxumResult<String> {

    let exp = match kind {
        "SESSION" => Utc::now() + ChronoDuration::minutes(60),
        "REFRESH" => Utc::now() + ChronoDuration::days(7),
        _         => Utc::now() + ChronoDuration::days(7),
    };

    let payload = JwtPayload { 
        id: user.id.to_hex(), 
        email: user.email.clone(),
        exp: exp.timestamp() as usize
    };

    Ok(sign_jwt(payload, key))?
}

pub fn new_cookie(kind: &str, name: &str, value: Option<&String>) -> Cookie<'static> {

    let exp = match kind {
        "SESSION" => time::Duration::minutes(60),
        "REFRESH" => time::Duration::days(7),
        _ => panic!("Invalid type of cookie")
    };

    let value = match value {
        Some(value) => value.clone(),
        None => String::new()
    };

    let mut cookie = Cookie::new(name, value);

    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_path("/");
    cookie.set_max_age(exp);

    cookie.into_owned()
}
