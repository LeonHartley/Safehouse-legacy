#[derive(Debug)]
pub enum ServerError {
    NoDbConnection,
    GenericError(String)
}

pub enum DbError {
    NoDbConnection,
    NotFound
}