
use bcrypt::hash;
use serde_json::{from_value, json, Value};
use std::collections::HashMap;
use mongodb::bson::{doc, oid::ObjectId};
use axum::{extract::{Path, State}, Json};
use axum_responses::{AxumResponse, HttpResponse, extra::ToJson};

use lxha_lib::{app::{constants::FRONTEND_SERVICE_URL, Context}, models::user::User, utils::{oid_from_str, reqwest::http_request}};
use crate::{incus_api::{delete::remove_instance, get::get_all_instances}, models::RegisterData};


pub async fn register_account(State(ctx): Context, Json(body): Json<RegisterData>) -> AxumResponse {

    if body.password != body.confirmPassword {
        return Err(HttpResponse::UNAUTHORIZED);
    }

    let mut conflicts_hash = HashMap::new();

    if ctx.users.find_one(doc! {"email": &body.email}).await?.is_some() {
        conflicts_hash.insert("email", "User with this email already exists");
    }

    if ctx.users.find_one(doc! {"username": &body.username}).await?.is_some() {
        conflicts_hash.insert("username", "User with this username already exists");
    }

    if !conflicts_hash.is_empty() {
        return Err(HttpResponse::JSON(409, "conflict", "conflicts", conflicts_hash.to_json()));
    }

    let user: User = User {
        id: ObjectId::new(),
        username: body.username,
        email: body.email,
        password: hash(body.password, 8).unwrap(),
        role: body.role,
        instances: vec![],
    };

    ctx.users.create(&user).await?;
    Ok(HttpResponse::CUSTOM(200, "Account registred successfully"))
}

pub async fn delete_account(State(ctx): Context,
    Path(oid): Path<String>) -> AxumResponse {

    let oid = oid_from_str(&oid)?;

    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesnt exists, cant delete the account"))
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

pub async fn update_account(State(ctx): Context, Path(oid): Path<String>, 
    Json(body): Json<Value>) -> AxumResponse {

    dbg!(&body);

    let oid = oid_from_str(&oid)?;

    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User (oid) doesnt exists, cant update it"))
    };

    let mut doc = doc! {};
    let valid_fields = vec!["username", "password", "email", "confirmPassword", "role"];
    let mut body_map: HashMap<String, String> = from_value(body).map_err(|_| HttpResponse::INTERNAL_SERVER_ERROR)?;
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

    let update = doc! {"$set": doc};
    ctx.users.update(&oid, update).await?;
    
    let mut email_updated = false;
    let mut response_message = "User updated succesfully";

    // if let Some(email) = body_map.get("email") {

    //     let update_email_url = format!("{}/dashboard/update-account-email", *FRONTEND_SERVICE_URL);
    //     let body = json!({ "email": &email, "url": update_email_url});
    //     let response = http_request("MAILER", "/email-change", "POST", None, body).await;
        
    //     let http_response = match response.status().as_u16() {
    //         200 => Ok(HttpResponse::OK),
    //         401 => Err(HttpResponse::BAD_REQUEST),
    //         _   => Err(HttpResponse::INTERNAL_SERVER_ERROR)
    //     };

    //     if let Ok(_) = http_response {
    //         // EL micro mailer se comunica con el endpoint /update_account_email y actualiza el email
    //         println!("http_response from the request between dashboard and mailer is => OK");
    //         email_updated = true;
    //     }

    // }

    if email_updated {
        println!("User email updated!");
        response_message = "User updated succesfully, Check your email and confirm the changes";
        // Cuando el usuario confirme, el mailer usara el endpoint /update-account-email
    }

    let updated = ctx.users.find_one_by_id(&oid).await?.unwrap();
    let profile = updated.into_json_user_data();

    Ok(HttpResponse::JSON(200, response_message, "user", profile))
}

pub async fn update_account_email(State(ctx): Context, Path(oid): Path<String>, 
    Json(body): Json<Value>) -> AxumResponse {
    
    let oid = oid_from_str(&oid)?;

    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesnt exists, cant get it"))
    };

    let body_map: HashMap<String, String> = from_value(body).map_err(|_| HttpResponse::INTERNAL_SERVER_ERROR)?;

    let mut conflicts_hash = HashMap::new();

    if ctx.users.find_one(doc! {"email": body_map.get("email")}).await?.is_some() {
        conflicts_hash.insert("email", "User with this email already exists");
    }

    if !conflicts_hash.is_empty() {
        return Err(HttpResponse::JSON(409, "conflict", "conflicts", conflicts_hash.to_json()));
    }

    let mut doc = doc! {};

    for (key, value) in &body_map {

        if key != "email" {
            return Err(HttpResponse::BAD_REQUEST); 
        }
        
        doc.insert(key, value);
    }


    let update = doc! {"$set": doc};
    ctx.users.update(&oid, update).await?;

    let updated = ctx.users.find_one_by_id(&oid).await?.unwrap();
    let profile = updated.into_json_user_data();
    
    Ok(HttpResponse::JSON(200, "User email updated succesfully", "user", profile)) 
}

pub async fn get_user(State(ctx): Context, Path(oid): Path<String>) -> AxumResponse {
    
    let oid = oid_from_str(&oid)?;

    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesnt exists, cant get it"))
    };

    Ok(HttpResponse::JSON(200, "User obtained succesfully", "user", user.into_json_user_data())) 
}

pub async fn get_users(State(ctx): Context) -> AxumResponse {
    
    let users = ctx.users.find().await?;

    Ok(HttpResponse::JSON(200, "Users obtained succesfully", "users", Value::from(users))) 
}