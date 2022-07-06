use rocket::response::Responder;
use std::error::Error;

#[derive(Responder, Debug)]
pub enum AuthError {
    #[response(status=401)]
    ExpiredToken(()), //trick
}

impl AuthError {
    pub fn expired() -> Self {
        Self::ExpiredToken(())
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            AuthError::ExpiredToken(str) => "JWT Credential has expired",
        };
        write!(f, "{}", printable)
    }
}

impl Error for AuthError {

}

#[cfg(test)]
mod test {
    use super::AuthError;

    #[test]
    fn test_auth_error() {
        let err = AuthError::expired();
        println!("debug: {:?}, normal: {}", err, err);
        assert_eq!(format!("{}", err), "JWT Credential has expired")
    }
}