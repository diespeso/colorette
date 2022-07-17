use rocket::response::Responder;
use std::error::Error;

#[derive(Responder, Debug)]
pub enum ImageUploadError {
    #[response(status=500)]
    InternalError(String),
    #[response(status=401)]
    AuthError(AuthError)
}

impl ImageUploadError {
    pub fn internal_error(err: String) -> Self {
        Self::InternalError(err)
    }

    pub fn auth_error(auth_error: AuthError) -> Self {
        Self::AuthError(auth_error)
    }
}

#[derive(Responder, Debug)]
pub enum AuthError {
    #[response(status=401)]
    ExpiredToken(()), //trick
    #[response(status=401)]
    UnreadableToken(String),
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

    pub fn unreadable(s: String) -> Self {
        Self::UnreadableToken(s)
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            AuthError::ExpiredToken(_) => "JWT Credential has expired",
            AuthError::NoCookie(s) => return write!(f, "Couldn't find Cookie: {}", s),
            AuthError::UnreadableToken(s) => return write!(f, "Coudln't verify JWT cause the claims or format are unexpected: {}", s),
        };
        write!(f, "{}", printable)
    }
}

impl Error for AuthError {

}

#[derive(Responder, Debug)]
pub enum SessionError {
    #[response(status=404)]
    UserNotFound(String),
    #[response(status=401)]
    AuthError(String)
}

impl SessionError {
    pub fn not_found(user: String) -> Self {
        Self::UserNotFound(user)
    }

    pub fn auth_error(authError: String) -> Self {
        Self::AuthError(authError)
    }
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self) //TODO: might be better to use a personalized for each enum case
    }
}

impl std::error::Error for SessionError {}

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