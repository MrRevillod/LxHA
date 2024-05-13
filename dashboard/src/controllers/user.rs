
use lxha_lib::models::user::User;
use lxha_lib::app::Context;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use axum::{extract::State, Extension, Json};
use axum_responses::{AxumResponse, HttpResponse, extra::ToJson};
use bcrypt::hash;
use serde_json::{from_value, Value};
use crate::models::RegisterData;
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

    // To-Do [] send email for validation
        
    //     let url = format!("{}/account/validate/{}/{}", 
    //         *CLIENT_ADDR, user.id.to_hex(), validation_token
    //     );

    //     send_acc_validation_email(&user.email, &url).await?;



    Ok(HttpResponse::CUSTOM(200, "Account registred successfully"))
}

pub async fn delete_account(State(ctx): Context,
    Extension(oid): Extension<ObjectId>) -> AxumResponse {

    ctx.users.delete(&oid);

    Ok(HttpResponse::CUSTOM(200, "Account deleted successfully"))
}

pub async fn update_account(State(ctx): Context, Extension(oid): Extension<ObjectId>, 
    Json(user): Json<User>, Json(body): Json<Value>) -> AxumResponse {

    let mut doc = doc! {};

    let valid_fields = vec!["id", "username", "email", "password", "confirm_password", "validated", "role", "instances"];

    let mut body_map: HashMap<String, String> = from_value(body)
        .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
    ;

    let mut conflicts_hash = HashMap::new();

    if ctx.users.find_one(doc! {"email": &user.email}).await?.is_some() {
        conflicts_hash.insert("email", "User with this email already exists");
    }

    if ctx.users.find_one(doc! {"username": &user.username}).await?.is_some() {
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

        doc.insert(key, value);
    }

    let update = doc! {"$set": doc};

    ctx.users.update(&oid, update);

    // let mut email_updated = false;

    // if let Some(email) = body_map.get("email") {

    //     let payload = (&oid.to_hex(), email);
    //     let exp = Utc::now() + ChronoDuration::hours(24);
    //     let secret = format!("{}{}", &JWT_SECRET.to_string(), &user.email);

    //     let token = sign_jwt(payload, &secret, exp)?;

    //     let url = format!("{}/account/update-email/{}/{}", *CLIENT_ADDR, oid.to_hex(), token);

    //     send_new_email_confirmation(&email, &url).await?;
    //     email_updated = true;
    // }

    // let mut response_msg = "Tu perfíl se ha actualizado";

    // if email_updated {
    //     response_msg = "Tu perfíl se ha actualizado, revisa tu email para confirmar el cambio";
    // }

    let updated = ctx.users.find_one(doc! {"_id": oid}).await?.unwrap();

    let profile = updated.into_profile();

    Ok(HttpResponse::JSON(200, "Tu perfíl se ha actualizado", "user", profile.to_json()))
}
