use rand::{distributions::Alphanumeric, Rng};
use bcrypt::hash;
use std::collections::HashMap;
use serde_json::{from_value, json, Value};
use mongodb::bson::{doc, oid::ObjectId};
use axum::{
    extract::{
        Path,
        State
    },
    Extension,
    Json
};
use axum_responses::{
    extra::ToJson,
    AxumResponse,
    HttpResponse
};
use lxha_lib::{
    app::{
        constants::{
            FRONTEND_SERVICE_URL,
            JWT_SECRET
        },
        Context
    },
    models::{
        token::EmailJwtPayload,
        user::{
            PublicProfile,
            Role,
            User
        }
    },
    utils::{
        jsonwebtoken::decode_jwt, 
        oid_from_str,
        reqwest::http_request
    }
};

use crate::models::RegisterData;

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

    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let user: User = User {
        id: ObjectId::new(),
        name: body.name,
        username: body.username,
        email: body.email,
        password: hash(password.clone(), 8).unwrap(),
        role: body.role,
        instances: vec![],
        n_instances: 0,
    };

    let body_mailer = json!({ "email": &user.email, "password": password });
    let mailer_res = http_request("MAILER", "/new-account", "POST", None, None, body_mailer).await;

    match mailer_res.status().as_u16() {
        200 => (),
        _   => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    let oid = user.id.to_hex();

    let body_project = json!({ "user": &oid });
    let mut admin_res = http_request("ADMIN", "/projects", "POST", None, None, body_project).await;
    
    match admin_res.status().as_u16() {
        200 => (),
        _   => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    admin_res = http_request("ADMIN", format!("/profiles/{}", oid).as_str(), "GET", None, None, json!({})).await;

    match admin_res.status().as_u16() {
        200 => (),
        _   => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    ctx.users.create(&user).await?;
    Ok(HttpResponse::CUSTOM(200, "Account registred successfully"))
}

pub async fn delete_account(State(ctx): Context,
    Path(id): Path<String>) -> AxumResponse {

    let oid = oid_from_str(&id)?;

    // let user
    let _ = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesn't exists"))
    };

    // Uncomment after

    // let instances = get_all_instances(user.username).await?;
    //
    // if !instances.is_empty() {
    //     for instance in instances {
    //         remove_instance(instance.name).await?;
    //     }
    // }
    
    let admin_res = http_request("ADMIN", format!("/projects/{}", id).as_str(), "DELETE", None, None, json!({})).await;

    match admin_res.status().as_u16() {
        200 => (),
        _   => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    ctx.users.delete(&oid).await?;

    Ok(HttpResponse::CUSTOM(200, "Account deleted successfully"))
}

pub async fn update_account(State(ctx): Context, Extension(user): Extension<PublicProfile>,
    Path(oid): Path<String>, Json(body): Json<Value>) -> AxumResponse {

    let oid = oid_from_str(&oid)?;
    let req_user_id = oid_from_str(&user.id)?;

    let req_user = match ctx.users.find_one_by_id(&req_user_id).await? {
        Some(user) => user,
        None => return Err(HttpResponse::UNAUTHORIZED)
    };

    if body.get("role").is_some() && req_user.role != Role::ADMINISTRATOR {
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
        conflicts_hash.insert("username", "Already in use");
    }

    if ctx.users.find_one(doc! {"email": body_map.get("email")}).await?.is_some() {
        conflicts_hash.insert("email", "Already in use");
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
            return Err(HttpResponse::NOT_ACCEPTABLE);
        }

        if key == "email" || key == "confirmPassword" {
            continue 
        }

        doc.insert(key, value);
    }

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

    ctx.users.update(&oid, doc! { "$set": doc }).await?;

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

    Ok(HttpResponse::OK)
}

pub async fn validate_email_update(State(ctx): Context, 
    Path((oid, token)): Path<(String, String)>) -> AxumResponse {

    let id = oid_from_str(&oid)?;

    let user = match ctx.users.find_one_by_id(&id).await? {
        Some(user) => user,
        None => return Err(HttpResponse::UNAUTHORIZED)
    };

    if oid != user.id.to_hex() {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    decode_jwt::<EmailJwtPayload>(&token, &JWT_SECRET)?;

    Ok(HttpResponse::OK)
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
