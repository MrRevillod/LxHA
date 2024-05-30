
use cookie::Cookie;
use tower_cookies::Cookies;

pub fn get_cookie_from_req(cookies: &Cookies, name: &str) -> Option<String> {
    cookies.get(name).map(|cookie| cookie.value().to_string())
}

pub fn new_cookie(kind: &str, name: &str, value: Option<&String>) -> Cookie<'static> {

    let exp = match kind {
        "SESSION" => time::Duration::minutes(2),
        "REFRESH" => time::Duration::days(7),
        _         => panic!("Invalid type of cookie")
    };

    let value = match value {
        Some(value) => value.clone(),
        None => String::new()
    };

    let mut cookie = Cookie::new(name, value);

    match kind {
        
        "SESSION" => {
            cookie.set_http_only(false);
        },
        
        "REFRESH" => {
            cookie.set_http_only(true);
        },
        _         => panic!("Invalid type of cookie")
    }
    
    cookie.set_path("/");
    cookie.set_max_age(exp);
    cookie.into_owned()
}
