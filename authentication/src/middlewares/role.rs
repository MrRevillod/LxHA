
use axum::Extension;
use axum::middleware::Next;

use axum_responses::HttpResponse;

use axum::extract::Request;
use axum::response::Response as MwResponse;

use lxha_lib::models::user::{Role, User};

pub async fn protected_role_validation(
    Extension(user): Extension<User>,
    req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    match user.role {
        Role::USER => Err(HttpResponse::UNAUTHORIZED),
        Role::ADMINISTRATOR => Ok(next.run(req).await)
    }
}

