
use bcrypt::hash;
use std::collections::HashMap;
use serde_json::{from_value, json, Value};
use mongodb::bson::{doc, oid::ObjectId};
use axum::{extract::{Path, State}, Extension, Json};
use axum_responses::{AxumResponse, HttpResponse, extra::ToJson};

use lxha_lib::{
    models::{token::EmailJwtPayload, user::{Role, User}}, 
    utils::{jsonwebtoken::decode_jwt, oid_from_str, reqwest::http_request},
    app::{constants::{DEFAULT_USER_PASSWORD, FRONTEND_SERVICE_URL, JWT_SECRET}, Context}, 
};

use crate::{incus_api::{delete::remove_instance, get::get_all_instances}, models::RegisterData};

pub async fn register_account(State(ctx): Context, Json(body): Json<RegisterData>) -> AxumResponse {

    let mut conflicts_hash = HashMap::new();

    if ctx.users.find_one(doc! {"email": &body.email}).await?.is_some() {
        conflicts_hash.insert("email", "User with this email already exists");
    }

    if ctx.users.find_one(doc! {"username": &body.username}).await?.is_some() {
        conflicts_hash.insert("username", "Username in use");
    }

    if !conflicts_hash.is_empty() {
        return Err(HttpResponse::JSON(409, "There are some conflicts in the user data", "conflicts", conflicts_hash.to_json()));
    }

    let user: User = User {
        id: ObjectId::new(),
        name: body.name,
        username: body.username,
        email: body.email,
        password: hash(DEFAULT_USER_PASSWORD.to_string(), 8).unwrap(),
        role: body.role,
        instances: vec![],
        n_instances: 0,
    };

    let body = json!({ "email": &user.email, "password": DEFAULT_USER_PASSWORD.to_string() });
    let mailer_res = http_request("MAILER", "/new-account", "POST", None, None, body);

    match mailer_res.await.status().as_u16() {
        200 => (),
        _   => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }
    
    ctx.users.create(&user).await?;
    Ok(HttpResponse::CUSTOM(200, "Account registred successfully"))
}

pub async fn delete_account(State(ctx): Context,
    Path(oid): Path<String>) -> AxumResponse {

    let oid = oid_from_str(&oid)?;

    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesn't exists"))
    };

    let instances = get_all_instances(user.username).await?;

    if !instances.is_empty() {
        for instance in instances {
            remove_instance(instance.name).await?;
        }
    }
    
    ctx.users.delete(&oid).await?;

    Ok(HttpResponse::CUSTOM(200, "Account deleted successfully"))
}

pub async fn update_account(State(ctx): Context, Extension(user): Extension<User>,
    Path(oid): Path<String>, Json(body): Json<Value>) -> AxumResponse {

    let oid = oid_from_str(&oid)?;

    if body.get("role").is_some() && user.role != Role::ADMINISTRATOR {
        return Err(HttpResponse::UNAUTHORIZED);
    };

    if let None = ctx.users.find_one_by_id(&oid).await? {
        return Err(HttpResponse::NOT_FOUND)
    };

    let mut doc = doc! {};

    let valid_fields = vec!["name", "username", "email", "password", "confirmPassword", "role"];

    let mut body_map: HashMap<String, String> = from_value(body)
        .map_err(|_| HttpResponse::INTERNAL_SERVER_ERROR)?
    ;

    let mut conflicts_hash = HashMap::new();

    if ctx.users.find_one(doc! {"username": body_map.get("username")}).await?.is_some() {
        conflicts_hash.insert("username", "User with this username already exists");
    }

    if ctx.users.find_one(doc! {"email": body_map.get("email")}).await?.is_some() {
        conflicts_hash.insert("email", "User with this email already exists");
    }

    if !conflicts_hash.is_empty() {
        return Err(HttpResponse::JSON(409, "conflict", "conflicts", conflicts_hash.to_json()));
    }

    if let Some(pwd) = body_map.get("password") {
        let encrypted = hash(pwd, 8).unwrap();
        body_map.insert("password".to_string(), encrypted);
    }
    
    for (key, value) in &body_map {
        
        if !valid_fields.contains(&key.as_str()) {
            return Err(HttpResponse::BAD_REQUEST);
        }

        if key == "email" || key == "confirmPassword" {
            continue 
        }

        doc.insert(key, value);
    }

    ctx.users.update(&oid, doc! { "$set": doc }).await?;
    
    let mut email_updated = false;
    let mut response_message = "User updated succesfully";

    if let Some(email) = body_map.get("email") {

        let update_email_url = format!("{}/account/update-email", *FRONTEND_SERVICE_URL);

        let body = json!({ "email": &email, "url": update_email_url});
        let response = http_request("MAILER", "/email-change", "POST", None, None, body).await;
        
        let http_response = match response.status().as_u16() {
            200 => Ok(HttpResponse::OK),
            401 => Err(HttpResponse::BAD_REQUEST),
            _   => Err(HttpResponse::INTERNAL_SERVER_ERROR)
        };

        if let Ok(_) = http_response {
            email_updated = true;
        }
    }

    if email_updated {
        response_message = "User updated succesfully, Check your email and confirm the changes";
    }

    let updated = ctx.users.find_one_by_id(&oid).await?.unwrap();
    let profile = updated.into_json_public_profile();

    Ok(HttpResponse::JSON(200, response_message, "user", profile))
}

pub async fn update_account_email(State(ctx): Context, 
    Path((oid, token)): Path<(String, String)>) -> AxumResponse {

    let id = oid_from_str(&oid)?;

    let user = match ctx.users.find_one_by_id(&id).await? {
        Some(user) => user,
        None => return Err(HttpResponse::UNAUTHORIZED)
    };

    if oid != user.id.to_hex() {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    let payload = decode_jwt::<EmailJwtPayload>(&token, &JWT_SECRET)?;

    ctx.users.update(&id, doc!{"$set": { "email": payload.email }}).await?;

    Ok(HttpResponse::ACCEPTED)
}

pub async fn get_user(State(ctx): Context, Path(oid): Path<String>) -> AxumResponse {
    
    let oid = oid_from_str(&oid)?;

    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::NOT_FOUND)
    };

    Ok(HttpResponse::JSON(200, "OK", "user", user.into_json_priv_profile())) 
}

pub async fn get_users(State(ctx): Context) -> AxumResponse {
    
    let users = ctx.users.find().await?;

    Ok(HttpResponse::JSON(200, "Users obtained succesfully", "users", users.to_json())) 
}
