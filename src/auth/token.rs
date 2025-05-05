use std::collections::BTreeMap;

use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{constants::time::ONE_DAY_IN_SECONDS, models::user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub username: String,
    iat: String,
    exp: String,
}

pub struct Token {}

impl Token {
    pub fn from_user(user: &User, secret_key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes())?;

        let mut claims = BTreeMap::new();
        let timestamp = Utc::now().timestamp();
        claims.insert("sub", user.clone().id.unwrap().to_string());
        claims.insert("username", user.clone().name);
        claims.insert("iat", timestamp.to_string());
        claims.insert("exp", (timestamp + ONE_DAY_IN_SECONDS).to_string());

        let token = claims.sign_with_key(&key)?;

        Ok(token)
    }

    pub fn parse(token: &str, secret_key: &str) -> Result<TokenClaims, Box<dyn std::error::Error>> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes())?;

        let token_data: jwt::Token<Header, TokenClaims, _> =
            token.verify_with_key(&key).map_err(|e| {
                log::error!("[Token] Error verifying token: {:?}", e);
                Box::new(jwt::Error::InvalidSignature)
            })?;

        let (_header, claims) = token_data.into();

        if claims.exp.parse::<i64>()? < Utc::now().timestamp() {
            log::info!("[Token] Expired: {}", claims.sub);
            return Err(Box::new(jwt::Error::InvalidSignature));
        }

        Ok(claims)
    }
}
