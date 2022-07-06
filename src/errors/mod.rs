use rocket::response::Responder;
use std::error::Error;

#[derive(Responder, Debug)]
pub enum AuthError {
    #[response(status=401)]
    ExpiredToken(()), //trick
    #[response(status=400)]
    NoCookie(String),
}

impl AuthError {
    pub fn expired() -> Self {
        Self::ExpiredToken(())
    }

    pub fn no_cookie(cookie_name: &str) -> Self {
        Self::NoCookie(cookie_name.to_owned())
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            AuthError::ExpiredToken(_) => "JWT Credential has expired",
            AuthError::NoCookie(s) => return write!(f, "Couldn't find Cookie: {}", s),
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
        let err = AuthError::no_cookie("jwt");
        println!("debug: {:?}, normal: {}", err, err);
        assert_eq!(format!("{}", err), "Couldn't find Cookie: jwt")
    }
}