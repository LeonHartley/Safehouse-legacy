use std::default::Default;
use crypto::sha2::Sha256;

use jwt::{Header, Registered, Token};
use error::AuthorisationError;

lazy_static! {
    static ref AUTH_SECRET: &'static str = "*GDF_LCkE=Aa,G:RQ6CHQXKt{@X/E#)e84N#rk+YNNC7j0mtOipWS#[igFg|ikj";
}

pub fn generate_token(issuer: &'static str, user_id: i64) -> String {
    let header: Header = Default::default();

    let claims = Registered {
        iss: Some(issuer.to_string()),
        sub: Some(format!("{}", user_id)),
        ..Default::default()
    };

    let token = Token::new(header, claims);
    let jwt = token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();

    format!("{}", jwt)
}

pub fn verify_token(token_str: &str) -> Result<i64, AuthorisationError> {
    let token = Token::<Header, Registered>::parse(&token_str).unwrap();
    let secret = AUTH_SECRET.as_bytes();

    if token.verify(&secret, Sha256::new()) {
        if let Some(user_id) = token.claims.sub {
            Ok(user_id.parse().unwrap())
        } else {
            Err(AuthorisationError::DataMissing)
        }
    } else {
        Err(AuthorisationError::Parse)
    }
}