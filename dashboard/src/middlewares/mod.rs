
use serde_json::Value;
use tower_cookies::Cookies;
use axum_responses::HttpResponse;

use lxha_lib::utils::{
    dbg::handle_error, 
    cookies::new_cookie, 
    reqwest::{http_request, parse_cookies}
};

use axum::{
    middleware::Next, extract::Request,
    response::Response as MwResponse
};

pub async fn authenticate_by_role(cookies: Cookies, 
    mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let cookie_jar = parse_cookies(cookies.clone());
    
    let response = http_request("AUTH", 
        "/validate-role", "POST", Some(cookie_jar), Value::Null).await
    ;

    let res_cookies = response.cookies();

    res_cookies.for_each(|cookie| {
        cookies.add(new_cookie("SESSION", cookie.name(), Some(&cookie.value().to_string())))
    });

    match response.status().as_u16() {

        200 => {
            
            let body: Value = response.json()
                .await.map_err(|e| handle_error(e))?
            ;

            let user = body.get("profile").unwrap().clone();
            req.extensions_mut().insert(user);
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
