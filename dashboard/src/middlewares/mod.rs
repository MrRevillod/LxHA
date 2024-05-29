
use serde_json::Value;
use tower_cookies::Cookies;
use axum_responses::{AxumResponse, HttpResponse};
use lxha_lib::{
    models::user::Profile, 
    utils::{dbg::handle_error, 
    reqwest::{http_request, parse_cookies}}
};

use axum::{
    extract::Request, middleware::Next, response::Response as MwResponse, Json
};

pub async fn authenticate_by_role(cookies: Cookies, 
    mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let cookie_jar = parse_cookies(cookies.clone());
    
    let response = http_request("AUTH", 
        "/validate-role", "POST", Some(cookie_jar), Value::Null).await
    ;

    match response.status().as_u16() {

        200 => {
            
            let profile = response.json::<Profile>()
                .await.map_err(|e| handle_error(e))?
            ;

            req.extensions_mut().insert(profile.id);
            req.extensions_mut().insert(profile);

            Ok(next.run(req).await)
        },

        500 => Err(HttpResponse::INTERNAL_SERVER_ERROR),
        _   => Err(HttpResponse::UNAUTHORIZED)
    }
}

// Validate by owner (para endpoints donde el usuario y el admin pueden acceder)

    // let response = http_request("AUTH", 
    //     "/validate-owner", "POST", Some(cookie_jar), Value::Null).await
    // ;
