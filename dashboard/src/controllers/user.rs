
use bcrypt::hash;
use std::collections::HashMap;
use mongodb::bson::{doc, oid::ObjectId};
use axum::{extract::{Path, State}, Json};
use axum_responses::{AxumResponse, HttpResponse, extra::ToJson};

use lxha_lib::{app::Context, models::user::User};
use crate::models::{RegisterData, UpdateUserData};

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
    Path(oid): Path<ObjectId>) -> AxumResponse {

    // TODO! y si el usuario tiene instancias creadas?
    // TODO! y si el usuario no existe?
    // TODO! y si la id no es un ObjectId?

    ctx.users.delete(&oid).await?;

    Ok(HttpResponse::CUSTOM(200, "Account deleted successfully"))
}

pub async fn update_account(State(ctx): Context, Path(oid): Path<ObjectId>, 
    Json(body): Json<UpdateUserData>) -> AxumResponse {

    // TODO! update_account debería ser dinamico, o sea recibir un 
    // json con los campos a actualizar, solo esos

    // En ese caso recomiendo hacer un middleware que valide los campos
    // y así se evita mezclar la lógica del controllador con la validación de datos

    dbg!(&body);
    
    // TODO! y si el usuario no existe?

    if body.password != body.confirmPassword {
        return Err(HttpResponse::UNAUTHORIZED);
    }    
    
    let mut doc = doc! {};
    let valid_fields = vec!["username", "password", "confirmPassword", "role"];

    // TODO! en rust la enum role no requiere un to_string, ya que de por si es un string

    let mut body_map = HashMap::from([
        ("username".to_string(), body.username.to_string()),
        ("password".to_string(), body.password.to_string()),
        ("confirmPassword".to_string(), body.confirmPassword.to_string()),
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

        if key == "email" || key == "confirmPassword" {
            continue 
        }

        doc.insert(key, value);
    }

    // TODO! y si viene un email en el body?
    // TODO! comunicarse con mailer

    let update = doc! {"$set": doc};
    ctx.users.update(&oid, update).await?;

    let updated = ctx.users.find_one_by_id(&oid).await?.unwrap();
    let profile = updated.into_json_profile();

    Ok(HttpResponse::JSON(200, "User updated succesfully", "user", profile))
}
