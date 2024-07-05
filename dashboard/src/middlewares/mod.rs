
use tower_cookies::Cookies;
use axum_responses::HttpResponse;
use serde_json::{from_value, Value};

use lxha_lib::{models::user::PublicProfile, utils::{
    cookies::new_cookie, 
    dbg::handle_internal_sv_error, 
    reqwest::{http_request, parse_cookies}
}};

use axum::{
    extract::Request,
    middleware::Next, 
    response::Response as MwResponse
};


pub async fn authenticate_by_session(cookies: Cookies, 
    mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let cookie_jar = parse_cookies(cookies.clone());
    
    let response = http_request("AUTH", "/validate-session", 
        "POST", None, Some(cookie_jar), Value::Null).await
    ;

    let res_cookies = response.cookies();

    res_cookies.for_each(|cookie| {
        cookies.add(new_cookie("SESSION", cookie.name(), Some(&cookie.value().to_string())))
    });

    match response.status().as_u16() {

        200 => {
            
            let body: Value = response.json()
                .await.map_err(|e| handle_internal_sv_error(e))?
            ;

            let profile = body.get("user").clone();

            let user = match profile {
                Some(profile) => match from_value::<PublicProfile>(profile.clone()) {
                    Ok(user) => user,
                    Err(_) => return Err(HttpResponse::INTERNAL_SERVER_ERROR),
                },
                None => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
            };

            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        },

        500 => Err(HttpResponse::INTERNAL_SERVER_ERROR),
        _   => Err(HttpResponse::UNAUTHORIZED)
    }
}


pub async fn authenticate_by_role(cookies: Cookies, 
    mut req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let cookie_jar = parse_cookies(cookies.clone());
    
    let response = http_request("AUTH", "/validate-role", 
        "POST", None, Some(cookie_jar), Value::Null).await
    ;

    let res_cookies = response.cookies();

    res_cookies.for_each(|cookie| {
        cookies.add(new_cookie("SESSION", cookie.name(), Some(&cookie.value().to_string())))
    });

    match response.status().as_u16() {

        200 => {
            
            let body: Value = response.json()
                .await.map_err(|e| handle_internal_sv_error(e))?
            ;

            let profile = body.get("user").clone();

            let user = match profile {
                Some(profile) => match from_value::<PublicProfile>(profile.clone()) {
                    Ok(user) => user,
                    Err(_) => return Err(HttpResponse::INTERNAL_SERVER_ERROR),
                },
                None => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
            };

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
                .await.map_err(|e| handle_internal_sv_error(e))?
            ;

            let profile = body.get("user").clone();

            let user = match profile {
                Some(profile) => match from_value::<PublicProfile>(profile.clone()) {
                    Ok(user) => user,
                    Err(_) => return Err(HttpResponse::INTERNAL_SERVER_ERROR),
                },
                None => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
            };

            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        },

        500 => Err(HttpResponse::INTERNAL_SERVER_ERROR),
        _   => Err(HttpResponse::UNAUTHORIZED)
    }
}
