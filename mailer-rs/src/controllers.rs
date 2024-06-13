
use serde_json::Value;
use axum::extract::Json;
use axum_responses::{AxumResponse,HttpResponse};
use crate::utils::{mailer::sender, templates::*};

pub async fn email_change(Json(body): Json<Value>) -> AxumResponse {
    
    let subject = "LxHA - Email update request";
    let user_email = body.get("email").unwrap().as_str().unwrap();
    let url = body.get("url").unwrap().as_str().unwrap();

    sender(change_email_template(url), subject, user_email)?;
    
    Ok(HttpResponse::OK)
}

pub async fn reset_password(Json(body): Json<Value>) -> AxumResponse {
    
    let subject = "LxHA - Password reset request";
    let user_email = body.get("email").unwrap().as_str().unwrap();
    let url = body.get("url").unwrap().as_str().unwrap();

    sender(reset_password_template(url), subject, user_email)?;
    
    Ok(HttpResponse::OK)
}

pub async fn contact_from_admin(Json(body): Json<Value>) -> AxumResponse {
    
    let subject = body.get("subject").unwrap().as_str().unwrap();
    let body_received = body.get("body").unwrap().as_str().unwrap();
    let user_email = body.get("receiverEmail").unwrap().as_str().unwrap();

    sender(contact_from_admin_template(body_received), subject, user_email)?;
    
    Ok(HttpResponse::OK)
}

pub async fn contact_from_user(Json(body): Json<Value>) -> AxumResponse {
    
    let subject = body.get("subject").unwrap().as_str().unwrap();
    let body_received = body.get("body").unwrap().as_str().unwrap();
    let user_email = body.get("receiverEmail").unwrap().as_str().unwrap();
    let from_name = body.get("fromName").unwrap().as_str().unwrap();
    let from_email = body.get("fromEmail").unwrap().as_str().unwrap();

    sender(contact_from_user_template(body_received, from_name, from_email), subject, user_email)?;
    
    Ok(HttpResponse::OK)
}

pub async fn new_account_message(Json(body): Json<Value>) -> AxumResponse {

    let subject = "LxHA - New Account";
    let user_email = body.get("email").unwrap().as_str().unwrap();
    let user_password = body.get("password").unwrap().as_str().unwrap();

    sender(new_account_message_template(user_email, user_password), subject, user_email)?;
    
    Ok(HttpResponse::OK)
}