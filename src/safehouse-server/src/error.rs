#[derive(Debug)]
pub enum ServerError {
    NoDbConnection,
    GenericError(String)
}

pub enum DbError {
    NoDbConnection,
    NotFound
}

pub trait ErrorString {
    fn get_string(&self) -> String;
}

pub enum AuthorisationError {
    Parse,
    Expired,
    Disabled,
    DataMissing
}

impl ErrorString for AuthorisationError {
    fn get_string(&self) -> String {
        match(&self) {
            Parse => "Failed to parse token".to_string(),
            _ => "Failed to authorise".to_string()
        }
    }
}