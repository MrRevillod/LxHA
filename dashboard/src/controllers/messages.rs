
use serde_json::json;
use axum::{extract::{Path, State}, Json};
use axum_responses::{AxumResponse, HttpResponse};

use lxha_lib::{app::{constants::LXHA_MAIL_ADRESS, Context}, 
    models::message::MessageFromView,
    utils::{oid_from_str, reqwest::http_request},
};

pub async fn contact_from_admin(State(ctx): Context,
    Path(id): Path<String>, Json(body): Json<MessageFromView>) -> AxumResponse {

    let receiver_id = oid_from_str(&id)?;

    let receiver = match ctx.users.find_one_by_id(&receiver_id).await? {
        Some(receiver) => receiver,
        None => return Err(HttpResponse::UNAUTHORIZED)
    };

    let mailer_body = json!({ 
        "receiverEmail": &receiver.email, 
        "subject": &body.subject, 
        "body": &body.body 
    });

    println!("mid cont");

    let mailer_res = http_request("MAILER", "/messages/from-admin", "POST", None, None, mailer_body).await;

    println!("after mailer");

    match mailer_res.status().as_u16() {
        200 => (),
        _   => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    };
    
    Ok(HttpResponse::OK)
}

pub async fn contact_from_user(Json(body): Json<MessageFromView>) -> AxumResponse {

    let mailer_body = json!({ 
        "receiverEmail": LXHA_MAIL_ADRESS.to_string(),
        "fromEmail": &body.from.email,
        "fromName": &body.from.name,
        "subject": &body.subject, 
        "body": &body.body 
    });

    let mailer_res = http_request("MAILER", "/messages/from-user", "POST", None, None, mailer_body).await;
    
    match mailer_res.status().as_u16() {
        200 => (),
        _   => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    };

    Ok(HttpResponse::OK)
}

