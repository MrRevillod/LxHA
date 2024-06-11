
use lxha_lib::{
    app::{constants::*, Context}, 
    models::token::EmailJwtPayload,
    utils::jsonwebtoken::{decode_jwt, new_token},
    utils::{oid_from_str, reqwest::http_request},
};

use bcrypt::hash;
use mongodb::bson::doc;
use serde_json::{json, Value};
use axum::{Json, extract::{State, Path}};
use axum_responses::{AxumResponse, HttpResponse};

pub async fn send_reset_password_email(State(ctx): Context, 
    Json(body): Json<Value>) -> AxumResponse {

    dbg!(&body);

    if !body.is_object() || body.get("email").is_none() {
        return Err(HttpResponse::BAD_REQUEST)
    }

    let email = body.get("email").unwrap().as_str().unwrap();

    // TODO! Check if the email is valid with a regex

    let user = match ctx.users.find_one(doc! { "email": email }).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(404, "User doesn't exists"))
    };

    let key = format!("{}{}", *JWT_SECRET, user.password);
    let token = new_token::<EmailJwtPayload>("ONETIME", &user, &key)?;

    let recovery_url = format!("{}/auth/reset-password/{}/{}", 
        *FRONTEND_SERVICE_URL, user.id.to_hex(), token
    );

    let body = json!({ "email": &email, "url": recovery_url });
    let response = http_request("MAILER", "/reset-password", "POST", None, None, body).await;

    match response.status().as_u16() {
        200 => Ok(HttpResponse::CUSTOM(200, "Email sent")),
        401 => Err(HttpResponse::BAD_REQUEST),
        _   => Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }
}

pub async fn reset_password_validation(State(ctx): Context,
    Path((id, token)): Path<(String, String)>) -> AxumResponse {

    let oid = oid_from_str(&id)?;

    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(404, "Invalid or expired URL"))
    };
    
    let key = format!("{}{}", *JWT_SECRET, user.password);
    
    decode_jwt::<EmailJwtPayload>(&token, &key)?;
    
    Ok(HttpResponse::OK)
}

pub async fn reset_password(State(ctx): Context,
    Path((id, token)): Path<(String, String)>, Json(body): Json<Value>) -> AxumResponse {

    let condition = !body.is_object() || 
        body.get("password").is_none() || 
        body.get("confirmPassword").is_none()
    ;

    if condition {
        return Err(HttpResponse::BAD_REQUEST)
    }

    let password = body.get("password").unwrap().as_str().unwrap();
    let confirm_pwd = body.get("confirmPassword").unwrap().as_str().unwrap();

    let password_mismatch = password != confirm_pwd;

    if password_mismatch {
        return Err(HttpResponse::BAD_REQUEST)
    }

    // TODO! Check if the password is valid with a regex
    
    let oid = oid_from_str(&id)?;
    
    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(404, "User doesn't exists"))
    };
    
    decode_jwt::<EmailJwtPayload>(&token, &format!("{}{}", *JWT_SECRET, user.password))?;

    ctx.users.update(&oid, doc! {
        "$set": { "password": hash(password, 8).unwrap() }}).await?
    ;

    Ok(HttpResponse::CUSTOM(200, "Password recover success"))
}
