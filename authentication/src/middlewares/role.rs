
use axum::middleware::Next;
use axum_responses::HttpResponse;

use axum::extract::Request;
use axum::response::Response as MwResponse;

use lxha_lib::models::user::{Role, User};

pub async fn protected_role_validation(
    req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let user = req.extensions().get::<User>().unwrap().clone();

    match user.role {
        Role::USER => Err(HttpResponse::UNAUTHORIZED),
        Role::ADMINISTRATOR => Ok(next.run(req).await)
    }
}

