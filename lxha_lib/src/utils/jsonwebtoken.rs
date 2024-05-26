
use jsonwebtoken::*;

use crate::models::user::User;
use crate::app::constants::JWT_SECRET;
use serde::{de::DeserializeOwned, Serialize};
use chrono::{Utc, Duration as ChronoDuration};
use axum_responses::{AxumResult, HttpResponse};
use crate::models::token::{Expirable, AuthJwtPayload};

pub fn sign_jwt<T>(payload: T, secret: &String) -> AxumResult<String>
where T: Serialize + Expirable {

    let token = encode(&Header::default(), &payload,
        &EncodingKey::from_secret(secret.as_bytes())
    );

    if let Err(_) = token {
        return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    Ok(token.unwrap())
}

pub fn decode_jwt<T>(token: &String, secret: &String) -> AxumResult<T>
where T: DeserializeOwned + Expirable {

    let payload = decode::<T>(token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default()
    );

    if let Err(_) = payload {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    let payload = payload.unwrap().claims;

    if (payload.exp() as i64) < Utc::now().timestamp() {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    Ok(payload)
}

/// Returns a new session token with the session/user id of the request 

pub fn new_session_token_from_refresh(refresh_token: &String) -> AxumResult<(String, String)> {

    let mut payload = decode_jwt::<AuthJwtPayload>(refresh_token, &JWT_SECRET)?;
    let exp = (Utc::now() + ChronoDuration::minutes(60)).timestamp() as usize;

    payload.set_exp(exp);

    Ok((sign_jwt(payload.clone(), &JWT_SECRET)?, payload.id))
}

pub fn new_token<T>(kind: &str, user: &User, key: &String) -> AxumResult<String> 
where T: From<User> + DeserializeOwned + Serialize + Expirable {

    let exp = match kind {
        "SESSION" => Utc::now() + ChronoDuration::minutes(60),
        "REFRESH" => Utc::now() + ChronoDuration::days(7),
        _         => Utc::now() + ChronoDuration::days(7),
    };

    let mut payload = T::from(user.clone());
    payload.set_exp(exp.timestamp() as usize);

    Ok(sign_jwt(payload, key)?)
}