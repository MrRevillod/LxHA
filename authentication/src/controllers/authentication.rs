
use lxha_lib::{
    app::{constants::JWT_SECRET, Context}, 
    models::{token::{AuthJwtPayload, Token}, user::User}, 
    utils::{cookies::{get_cookie_from_req, new_cookie}, jsonwebtoken::new_token}
};

use bcrypt::verify;
use mongodb::bson::doc;
use serde_json::Value;
use tower_cookies::Cookies;
use axum::{extract::State, Extension, Json};
use axum_responses::{AxumResponse, HttpResponse};

pub async fn login_controller(cookies: Cookies, 
    State(ctx): Context, Json(body): Json<Value>) -> AxumResponse {

    if !body.is_object() || body.get("email").is_none() || body.get("password").is_none(){
        return Err(HttpResponse::BAD_REQUEST)
    }

    let email = body.get("email").unwrap().as_str().unwrap();
    let password = body.get("password").unwrap().as_str().unwrap();

    let filter = doc! { "email": email };
    
    let user = match ctx.users.find_one(filter).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(401, "Invalid credentials"))
    };
    
    if !verify(password, &user.password).unwrap() {
        return Err(HttpResponse::CUSTOM(401, "Invalid credentials"))
    }
    
    let token = new_token::<AuthJwtPayload>("SESSION", &user, &JWT_SECRET)?;
    let refresh = new_token::<AuthJwtPayload>("REFRESH", &user, &JWT_SECRET)?;

    let session_cookie = new_cookie("SESSION", "session", Some(&token));
    let refresh_cookie = new_cookie("REFRESH", "refresh", Some(&refresh));

    cookies.add(session_cookie);
    cookies.add(refresh_cookie);

    let profile = user.into_json_profile();

    Ok(HttpResponse::JSON(200, "Login success", "user", profile))
}

pub async fn logout_controller(cookies: Cookies, 
    State(ctx): Context, Extension(user): Extension<User>,
    Extension(token): Extension<String>) -> AxumResponse {

    let refresh_cookie = get_cookie_from_req(&cookies, "refresh");

    let refresh_token = match refresh_cookie {
        Some(refresh_token) => refresh_token,
        None => String::new()
    };

    let mut token_struct = Token {
        token, user_id: user.id
    };

    ctx.tokens.save(&token_struct).await?;
    token_struct.token = refresh_token;
    ctx.tokens.save(&token_struct).await?;

    let session_cookie = new_cookie("SESSION", "session", None);
    let refresh_cookie = new_cookie("REFRESH", "refresh", None);

    cookies.remove(session_cookie);
    cookies.remove(refresh_cookie);

    Ok(HttpResponse::CUSTOM(200, "The session has been closed"))
}

pub async fn authenticate(Extension(user): Extension<User>) -> AxumResponse {
    Ok(HttpResponse::JSON(200, "Authenticated", "user", user.into_json_profile()))
}
