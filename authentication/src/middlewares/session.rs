
use crate::utils::session::*;

use mongodb::bson::doc;
use tower_cookies::Cookies;
use axum_responses::HttpResponse;

use lxha_lib::{
    utils::oid_from_str,
    app::{Context, constants::JWT_SECRET}
};

use axum::{
    middleware::Next,
    extract::{Request, State},
    response::Response as MwResponse
};

pub async fn session_validation(cookies: Cookies, 
    State(ctx): Context, mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let mut token = cookies.get("token").map(|cookie| cookie.value().to_string());

    let user_id = match &token {
        
        Some(token) => decode_jwt(&token, &JWT_SECRET)?.id,

        None => {

            let refresh_token = match cookies.get("refresh") {
                Some(refresh_token) => refresh_token.value().to_string(),
                None => return Err(HttpResponse::UNAUTHORIZED)
            };

            let (new_token, user_id) = new_token_from_refresh(&refresh_token)?;
            let session_cookie = new_cookie("SESSION", "token", Some(&new_token));

            cookies.add(session_cookie);

            token = Some(new_token);
            user_id
        }
    };

    if ctx.tokens.find_one(doc! { "token": &token }).await?.is_some() {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    let id = oid_from_str(&user_id)?;

    let user = match ctx.users.find_one_by_id(&id).await? {
        Some(user) => user,
        None   => return Err(HttpResponse::UNAUTHORIZED)
    };

    req.extensions_mut().insert(user);
    req.extensions_mut().insert(token.unwrap());

    Ok(next.run(req).await)
}

