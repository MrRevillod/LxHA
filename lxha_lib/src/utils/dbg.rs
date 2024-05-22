
use axum_responses::HttpResponse;

pub fn handle_error<T>(error: T) -> HttpResponse
where T: std::error::Error {

    eprint!("Error: {}", error);

    HttpResponse::INTERNAL_SERVER_ERROR
}