
use axum::{
    middleware::Next,
    extract::{Path, Request, State}, 
    response::Response as MwResponse
};

use axum_responses::HttpResponse;
use mongodb::bson::oid::ObjectId;

use lxha_lib::{utils::*, app::Context, models::user::*};

pub async fn _is_valid_id(State(ctx): Context, Path(id): Path<String>, 
    mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let oid = oid_from_str(&id)?;

    if ctx.users.find_one_by_id(&oid).await?.is_none() {
        return Err(HttpResponse::CUSTOM(404, "Resource not found"))
    }

    req.extensions_mut().insert(oid);
    Ok(next.run(req).await)
}

pub async fn _is_valid_id_and_token(State(ctx): Context,
    Path((id, token)): Path<(String, String)>, 
    mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let oid = oid_from_str(&id)?;

    if ctx.users.find_one_by_id(&oid).await?.is_none() {
        return Err(HttpResponse::CUSTOM(404, "Resource not found"))
    }

    req.extensions_mut().insert(oid);
    req.extensions_mut().insert(token);

    Ok(next.run(req).await)
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
