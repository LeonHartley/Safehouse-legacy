
#[derive(RustcDecodable, RustcEncodable)]
pub struct AuthorisationRequest {
    pub username: String,
    pub password: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct UserAccount {
    pub username: String,
    pub avatar: Vec<i8>
}
