use serde_json::Value;
use axum::extract::Json;
use axum_responses::{AxumResponse,HttpResponse};
use crate::utils::{mailer::sender, templates::*};
// use lxha_lib::app::constants::{LXHA_MAIL_ADRESS, LXHA_MAIL_PASSWORD, LXHA_MAIL_HOST};
// use std::process::Command;
// use std::path::Path;
// use lettre_email::Email;
// use lettre::{Message, SmtpTransport, Transport};
// use lettre::transport::smtp::authentication::Credentials;
// use mime;

// Handler function for email change requests
pub async fn email_change(Json(body): Json<Value>) -> AxumResponse {
    
    let subject = "LxHA - Email update request";
    let user_email = body.get("email").unwrap().as_str().unwrap();
    let url = body.get("url").unwrap().as_str().unwrap();
    // Send the email using the sender function with the change email template
    sender(change_email_template(url), subject, user_email)?;
    // Return an OK response if email is sent successfully
    Ok(HttpResponse::OK)
}

// Handler function for password reset requests
pub async fn reset_password(Json(body): Json<Value>) -> AxumResponse {
    
    let subject = "LxHA - Password reset request";
    let user_email = body.get("email").unwrap().as_str().unwrap();
    let url = body.get("url").unwrap().as_str().unwrap();
    // Send the email using the sender function with the password reset template
    sender(reset_password_template(url), subject, user_email)?;
    // Return an OK response if email is sent successfully
    Ok(HttpResponse::OK)
}

// Handler function for sending emails from admin to users
pub async fn contact_from_admin(Json(body): Json<Value>) -> AxumResponse {
    
    let subject = body.get("subject").unwrap().as_str().unwrap();
    let body_received = body.get("body").unwrap().as_str().unwrap();
    let user_email = body.get("receiverEmail").unwrap().as_str().unwrap();
    // Send the email using the sender function with the from admin template
    sender(contact_from_admin_template(body_received), subject, user_email)?;
    // Return an OK response if email is sent successfully
    Ok(HttpResponse::OK)
}

// Handler function for sending emails from users to admins
pub async fn contact_from_user(Json(body): Json<Value>) -> AxumResponse {
    
    let subject = body.get("subject").unwrap().as_str().unwrap();
    let body_received = body.get("body").unwrap().as_str().unwrap();
    let user_email = body.get("receiverEmail").unwrap().as_str().unwrap();
    let from_name = body.get("fromName").unwrap().as_str().unwrap();
    let from_email = body.get("fromEmail").unwrap().as_str().unwrap();
    // Send the email using the sender function with the from user template
    sender(contact_from_user_template(body_received, from_name, from_email), subject, user_email)?;
    // Return an OK response if email is sent successfully
    Ok(HttpResponse::OK)
}

// Handler function for sending new account creation emails
pub async fn new_account_message(Json(body): Json<Value>) -> AxumResponse {

    let subject = "LxHA - New Account";
    let user_email = body.get("email").unwrap().as_str().unwrap();
    let user_password = body.get("password").unwrap().as_str().unwrap();
    // Send the email using the sender function with the new account message template
    sender(new_account_message_template(user_email, user_password), subject, user_email)?;
    // Return an OK response if email is sent successfully
    Ok(HttpResponse::OK)
}

pub async fn new_instance_message(Json(body): Json<Value>) -> AxumResponse {

    let subject = "LxHA - New instance";
    let user_email = body.get("email").unwrap().as_str().unwrap();
    let password = body.get("password").unwrap().as_str().unwrap();
    // let private_key = body.get("private_key").unwrap().as_str().unwrap();
    // Send the email using the sender function with the new account message template

    sender(new_instance_message_template(password), subject, user_email)?;
    // Return an OK response if email is sent successfully
    Ok(HttpResponse::OK)
}



















    // Command::new("sh")
    //     .arg("-c")
    //     .arg(format!("echo -e '{}' > key.txt", private_key))
    //     .output()
    //     .expect("Error: Failed to execute echo");

    // let email = Email::builder()
    //     .to(user_email)
    //     .from(LXHA_MAIL_ADRESS.clone())
    //     .subject(subject)
    //     .html(new_instance_message_template(password, private_key))
    //     .attachment_from_file(Path::new("key.txt"), None, &mime::TEXT_PLAIN)
    //     .unwrap()
    //     .build()
    //     .unwrap();


    // let creds = Credentials::new(
    //     LXHA_MAIL_ADRESS.clone(),
    //     LXHA_MAIL_PASSWORD.clone(),
    // );

    // let transporter = SmtpTransport::relay(&LXHA_MAIL_HOST).unwrap().credentials(creds).build();
    // 
    // match transporter.send(&email) {
    //     Ok(_) => return Ok(HttpResponse::CUSTOM(200, "Email sent successfully!")),
    //     Err(e) => {
    //         dbg!(&e);
    //         return Err(HttpResponse::CUSTOM(400, "Email was not sent"))},
    // }
