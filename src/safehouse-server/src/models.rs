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

#[derive(RustcDecodable, RustcEncodable)]
pub struct RealtimeAuthRequest {
    pub token: String,
    pub key: String
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct UserAccount {
    pub id: i64,
    pub username: String,
    pub avatar: String,
    pub status: String,
    pub date_created: String, 
    pub date_active: String
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ContactStatus {
    pub id: i64,
    pub status: UserStatus,
    pub key: Option<String>
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ChatMessage {
    pub sender: i64,
    pub user_id: i64,
    pub message: String
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub enum UserStatus {
    Online,
    Offline
}

impl ToJson for ChatMessage {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("sender".to_string(), self.sender.to_json());
        map.insert("user_id".to_string(), self.user_id.to_json());
        map.insert("message".to_string(), self.message.to_json());

        Json::Object(map)
    }
}


impl ToJson for ContactStatus {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("id".to_string(), self.id.to_json());
        map.insert("status".to_string(), self.status.to_json());

        if let Some(ref key) = self.key {
            map.insert("key".to_string(), key.to_json());
        }

        Json::Object(map)
    }
}

impl ToJson for UserStatus {
    fn to_json(&self) -> Json {
        match(self) {
            Online => "online".to_json(),
            Offline => "offline".to_json()
        }
    }
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
        map.insert("status".to_string(), self.status.to_json());
        
        Json::Object(map)
    }
}