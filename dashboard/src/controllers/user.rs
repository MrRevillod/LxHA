
use lxha_lib::models::user::User;
use lxha_lib::app::Context;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use axum::{extract::{Path, State}, Json};
use axum_responses::{AxumResponse, HttpResponse, extra::ToJson};
use bcrypt::hash;
use crate::models::{RegisterData, UpdateUserData};
use std::collections::HashMap;


pub async fn register_account(State(ctx): Context, Json(body): Json<RegisterData>) -> AxumResponse {

    if body.password != body.confirm_password {
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
        validated: false, 
        role: body.role,
        instances: vec![],
    };


    ctx.users.create(&user).await?;

    Ok(HttpResponse::CUSTOM(200, "Account registred successfully"))
}

pub async fn delete_account(State(ctx): Context,
    Path(oid): Path<ObjectId>) -> AxumResponse {

    ctx.users.delete(&oid).await?;

    Ok(HttpResponse::CUSTOM(200, "Account deleted successfully"))
}

pub async fn update_account(State(ctx): Context, Path(oid): Path<ObjectId>, 
    Json(body): Json<UpdateUserData>) -> AxumResponse {

    if body.password != body.confirm_password {
        return Err(HttpResponse::UNAUTHORIZED);
    }    
    
    let mut doc = doc! {};

    let valid_fields = vec!["username", "password", "confirm_password", "validated", "role"];


    let mut body_map = HashMap::from([
        ("username".to_string(), body.username.to_string()),
        ("password".to_string(), body.password.to_string()),
        ("confirm_password".to_string(), body.confirm_password.to_string()),
        ("validated".to_string(), body.validated.to_string()),
        ("role".to_string(), body.role_to_string()),
    ]); 


    let mut conflicts_hash = HashMap::new();


    if ctx.users.find_one(doc! {"username": body_map.get("username")}).await?.is_some() {
        conflicts_hash.insert("username", "User with this username already exists");
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

        if key == "email" || key == "confirm_password" {
            continue 
        }


        if key == "validated" && value == "true" {
            doc.insert(key, true);
            continue
        }

        if key == "validated" && value == "false" {
            doc.insert(key, false);
            continue
        }

        doc.insert(key, value);

    }

    let update = doc! {"$set": doc};

    ctx.users.update(&oid, update).await?;

    let updated = ctx.users.find_one_by_id(&oid).await?.unwrap();

    let profile = updated.into_json_profile();

    Ok(HttpResponse::JSON(200, "User updated succesfully", "user", profile))
}