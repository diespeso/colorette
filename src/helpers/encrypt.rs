use sha2::{Sha256, Digest};
use jsonwebtoken::{errors::Result, encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use rocket::serde::{Deserialize, Serialize, json::Json, json::json};


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

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthToken {
    user_id: i32,
    username: String
}

impl AuthToken {
    pub fn new(user_id: i32, username: String) -> Self {
        Self{user_id, username}
    }
}

pub fn sign_token(payload: impl Serialize, secret: &str) -> Result<String> {
    encode(&Header::default(), &payload, &EncodingKey::from_secret(secret.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::{sha256, sign_token, AuthToken};
    #[test]
    fn test_sha256() {
        assert_eq!(sha256("string!"),
        "f9435df100262491e8918dfffc1152072f806b8a161c932219d5d76c73ecab23")
    }
    #[test]
    fn test_sign_token() {
        let token = sign_token(AuthToken{user_id: 0, username: "juanito".to_string()}, "secreto");
        println!("{}", token.unwrap());
    }
}