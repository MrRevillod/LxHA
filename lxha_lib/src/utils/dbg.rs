
use axum_responses::HttpResponse;

pub fn handle_internal_sv_error<T>(error: T) -> HttpResponse
where T: std::error::Error + std::fmt::Debug {

    eprint!("Error: {:?}", error);

    HttpResponse::INTERNAL_SERVER_ERROR
}