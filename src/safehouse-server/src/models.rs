use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};

#[derive(RustcDecodable, RustcEncodable)]
pub struct AuthorisationRequest {
    pub username: String,
    pub password: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct AuthorisationResponse {
    pub token: String,
    pub data: UserAccount
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct UserAccount {
    pub id: i64,
    pub username: String,
    pub avatar: String,
    pub date_created: String, 
    pub date_active: String
}

impl ToJson for AuthorisationResponse {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("token".to_string(), self.token.to_json());
        map.insert("data".to_string(), self.data.to_json());

        Json::Object(map)
    }    
}

impl ToJson for UserAccount {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("id".to_string(), self.id.to_json());
        map.insert("username".to_string(), self.username.to_json());
        map.insert("avatar".to_string(), self.avatar.to_json());
        
        Json::Object(map)
    }
}