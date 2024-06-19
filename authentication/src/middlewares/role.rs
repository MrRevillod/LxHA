
use axum::middleware::Next;
use axum::extract::Request;
use axum_responses::HttpResponse;
use mongodb::bson::oid::ObjectId;
use lxha_lib::models::user::{Role, User};
use axum::response::Response as MwResponse;

pub async fn protected_role_validation(
    req: Request, next: Next) -> Result<MwResponse, HttpResponse> {
    let user = req.extensions().get::<User>().unwrap().clone();

    match user.role {
        Role::USER => Err(HttpResponse::UNAUTHORIZED),
        Role::ADMINISTRATOR => Ok(next.run(req).await)
    }
}

pub async fn owner_validation(req: Request, 
    next: Next) -> Result<MwResponse, HttpResponse> {
    
    let oid = req.extensions().get::<ObjectId>().unwrap().clone();
    let user = req.extensions().get::<User>().unwrap().clone();

    if user.id != oid || user.role != Role::ADMINISTRATOR {
        return Err(HttpResponse::UNAUTHORIZED)
    }

    Ok(next.run(req).await)
}