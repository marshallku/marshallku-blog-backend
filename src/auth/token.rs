use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use crate::models::user::User;

pub struct Token {}

impl Token {
    pub fn from_user(user: &User) -> Result<String, Box<dyn std::error::Error>> {
        let secret_key = std::env::var("JWT_SECRET").unwrap();
        let key: Hmac<Sha256> = Hmac::new_from_slice(&secret_key.into_bytes())?;

        let mut claims = BTreeMap::new();
        claims.insert("sub", user.clone().id.unwrap().to_string());
        claims.insert("username", user.clone().name);

        let token = claims.sign_with_key(&key)?;

        Ok(token)
    }
}
