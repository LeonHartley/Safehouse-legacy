
#[derive(RustcDecodable, RustcEncodable)]
pub struct AuthorisationRequest {
    pub username: String,
    pub password: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct UserAccount {
    pub id: i64,
    pub username: String,
    pub avatar: String
}
