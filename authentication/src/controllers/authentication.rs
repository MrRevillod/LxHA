
use lxha_lib::models::user::User;
use lxha_lib::models::token::Token;
use lxha_lib::app::{Context, constants::JWT_SECRET};
use crate::utils::session::{new_token, new_cookie};

use bcrypt::verify;
use mongodb::bson::doc;
use tower_cookies::Cookies;
use axum::{extract::State, Extension, Json};
use axum_responses::{AxumResponse, HttpResponse};

use crate::models::LoginData;

pub async fn login_controller(cookies: Cookies, 
    State(ctx): Context, Json(body): Json<LoginData>) -> AxumResponse {

    let filter = doc! { "email": &body.email };
    
    let user = match ctx.users.find_one(filter).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(401, "Invalid credentials"))
    };
    
    if !verify(&body.password, &user.password).unwrap() {
        return Err(HttpResponse::CUSTOM(401, "Invalid credentials"))
    }

    if !user.validated {
        return Err(HttpResponse::CUSTOM(401, "Account not validated"))
    }
    
    let token = new_token("SESSION", &user, &JWT_SECRET)?;
    let refresh = new_token("REFRESH", &user, &JWT_SECRET)?;

    let session_cookie = new_cookie("SESSION", "token", Some(&token));
    let refresh_cookie = new_cookie("REFRESH", "refresh", Some(&refresh));

    cookies.add(session_cookie);
    cookies.add(refresh_cookie);

    let profile = user.into_json_profile();

    Ok(HttpResponse::JSON(200, "Login success", "user", profile))
}

pub async fn logout_controller(cookies: Cookies, 
    State(ctx): Context, Extension(user): Extension<User>,
    Extension(token): Extension<String>) -> AxumResponse {

    let refresh_cookie = cookies.get("refresh")
        .map(|cookie| cookie.value().to_string())
    ;

    let refresh_token = match refresh_cookie {
        Some(refresh_token) => refresh_token,
        None => return Err(HttpResponse::UNAUTHORIZED)
    };

    let mut token_struct = Token {
        token, user_id: user.id
    };

    ctx.tokens.save(&token_struct).await?;
    token_struct.token = refresh_token;
    ctx.tokens.save(&token_struct).await?;

    let session_cookie = new_cookie("SESSION", "token", None);
    let refresh_cookie = new_cookie("REFRESH", "refresh", None);

    cookies.remove(session_cookie);
    cookies.remove(refresh_cookie);

    Ok(HttpResponse::OK)
}

pub async fn authenticate(Extension(user): Extension<User>) -> AxumResponse {
    Ok(HttpResponse::JSON(200, "Authenticated", "user", user.into_json_profile()))
}
