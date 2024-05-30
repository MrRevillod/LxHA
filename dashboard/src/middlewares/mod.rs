
use serde_json::Value;
use tower_cookies::Cookies;
use axum_responses::HttpResponse;

use lxha_lib::utils::{
    dbg::handle_error, 
    cookies::new_cookie, 
    reqwest::{http_request, parse_cookies}
};

use axum::{
    extract::Request,
    middleware::Next, 
    response::Response as MwResponse
};

pub async fn authenticate_by_role(cookies: Cookies, 
    mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let client_ip: String = req.headers().get("x-forwarded-by")
        .map(|ip| ip.to_str().unwrap().to_string())
        .unwrap_or(String::new())
    ;

    let cookie_jar = parse_cookies(cookies.clone());
    
    let response = http_request("AUTH", "/validate-role", 
        "POST", Some(client_ip.clone()), Some(cookie_jar), Value::Null).await
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

            let user = body.get("user").unwrap().clone();
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        },

        500 => Err(HttpResponse::INTERNAL_SERVER_ERROR),
        _   => Err(HttpResponse::UNAUTHORIZED)
    }
}

pub async fn authenticate_by_owner(cookies: Cookies, 
    mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let cookie_jar = parse_cookies(cookies.clone());
    
    let response = http_request("AUTH", "/validate-owner", 
        "POST", None, Some(cookie_jar), Value::Null).await
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

            let user = body.get("user").unwrap().clone();
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        },

        500 => Err(HttpResponse::INTERNAL_SERVER_ERROR),
        _   => Err(HttpResponse::UNAUTHORIZED)
    }
}
