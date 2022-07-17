use sha2::{Sha256, Digest};
use jsonwebtoken::{errors::Result, TokenData, encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use rocket::serde::{Deserialize, Serialize, json::Json, json::json, self};

use crate::helpers::{StdError, StdResult};
use crate::errors;


/// Generates a Hashed hex string of 256 bytes.
pub fn sha256(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().iter().map(
        |byte| {
            format!("{:02x}", byte)
        }
    ).collect()
}

pub fn verify_sha256(hashed: &String, string: &String) -> bool {
    if hashed == &sha256(string) {
        return true
    }
    return false
} 

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthToken {
    user_id: i32,
    pub username: String,
    exp: usize,
}

impl AuthToken {
    pub fn new(user_id: i32, username: String) -> Self {
        Self{user_id, username, exp: 1659999912363} //set date, should fin a better way
    }
}

pub fn sign_token(payload: impl Serialize, secret: &str) -> Result<String> {
    encode(&Header::default(), &payload, &EncodingKey::from_secret(secret.as_ref()))
}

/**
 * Most restrictive function ever methinks
 */
pub fn verify_token(token: &str, secret: &str) -> StdResult<TokenData<AuthToken>, Box<StdError>> {
    //println!("token: {}", token);
    let payload = decode::<AuthToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));
    //println!("{:?}", payload);
    if let Ok(token_data) = payload {
        Ok(token_data)
    } else {
        Err(Box::new(
            errors::AuthError::unreadable(token.to_string())
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{sha256, sign_token, AuthToken};
    use std::{time::{Duration, SystemTime}, alloc::System};
    #[test]
    fn test_sha256() {
        assert_eq!(sha256("string!"),
        "f9435df100262491e8918dfffc1152072f806b8a161c932219d5d76c73ecab23")
    }
    #[test]
    fn test_sign_token() {
        let token = sign_token(AuthToken{user_id: 0, username: "juanito".to_string(), exp:0}, "secreto");
        println!("{}", token.unwrap());
    }

    #[test]
    fn test_epoch_time() {
        /*let now = SystemTime::now();
        println!("now: {:?}", now);
        let dur = now + Duration::new(6, 0);
        println!("dur: {:?}", dur);
        println!("normal dur: {:?}", Duration::new(6, 0))*/
    }
}