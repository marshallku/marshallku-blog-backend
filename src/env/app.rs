use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Env {
    pub port: u16,
    pub host: Cow<'static, str>,
    pub jwt_secret: Cow<'static, str>,
    pub cookie_domain: Cow<'static, str>,
}

impl Env {
    pub fn new() -> Self {
        let port = match std::env::var("PORT") {
            Ok(port) => port.parse().unwrap_or(8080),
            Err(_) => 8080,
        };
        let host = match std::env::var("HOST") {
            Ok(host) => Cow::Owned(host),
            Err(_) => Cow::Owned("http://localhost/".to_string()),
        };
        let jwt_secret = match std::env::var("JWT_SECRET") {
            Ok(jwt_secret) => Cow::Owned(jwt_secret),
            Err(_) => panic!("JWT_SECRET is not set"),
        };
        let cookie_domain = match std::env::var("COOKIE_DOMAIN") {
            Ok(cookie_domain) => Cow::Owned(cookie_domain),
            Err(_) => Cow::Owned("localhost".to_string()),
        };

        Self {
            port,
            host,
            jwt_secret,
            cookie_domain,
        }
    }
}
