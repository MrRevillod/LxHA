
use mongodb::bson::doc;
use tower_cookies::Cookies;
use axum_responses::HttpResponse;

use lxha_lib::{
    models::token::AuthJwtPayload, 
    app::{constants::JWT_SECRET, Context}, 
    utils::{cookies::{get_cookie_from_req, new_cookie}, 
    jsonwebtoken::{decode_jwt, new_session_token_from_refresh}, oid_from_str}
};

use axum::{
    middleware::Next,
    extract::{Request, State},
    response::Response as MwResponse
};

pub async fn session_validation(cookies: Cookies, 
    State(ctx): Context, mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let mut token = get_cookie_from_req(&cookies, "session");
    let refresh = get_cookie_from_req(&cookies, "refresh");

    let token_clone = token.clone();

    if refresh.is_none() {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    let mut session_from_refresh = |refresh: &String| -> Result<String, HttpResponse> {

        let (new_token, user_id) = new_session_token_from_refresh(&refresh)?;
        let session_cookie = new_cookie("SESSION", "session", Some(&new_token));

        cookies.add(session_cookie);
        token = Some(new_token);

        Ok(user_id)
    };
    
    let user_id = match &token_clone {

        Some(session) => {

            match decode_jwt::<AuthJwtPayload>(&session, &JWT_SECRET) {
                Ok(payload) => payload.id,
                Err(_) => session_from_refresh(&refresh.unwrap())?
            }
        },

        None => session_from_refresh(&refresh.unwrap())?
    };

    println!("User ID: {}", &user_id);

    if ctx.tokens.find_one(doc! { "token": &token }).await?.is_some() {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    let id = oid_from_str(&user_id)?;

    let user = match ctx.users.find_one_by_id(&id).await? {
        Some(user) => user,
        None   => return Err(HttpResponse::UNAUTHORIZED)
    };
    
    println!("User: {:?}", &user);
    req.extensions_mut().insert(id);
    req.extensions_mut().insert(user);
    req.extensions_mut().insert(token.unwrap());


    Ok(next.run(req).await)
}

