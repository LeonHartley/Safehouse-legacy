use mysql::{Pool};

use models::{AuthorisationRequest, UserAccount};

use std::env;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref POOL: Arc<Mutex<Pool>> = Arc::new(Mutex::new(Pool::new({
            // fetch from another source, e.g env vars
            "mysql://root:password@localhost:3306/safehouse"
        }).unwrap()));
}

#[derive(Debug)]
pub struct DatabaseCtx {}

pub trait UserRepo {
    fn find_user_by_auth(request: AuthorisationRequest) -> Result<UserAccount, ()>;
}

impl UserRepo for DatabaseCtx {
    fn find_user_by_auth(request: AuthorisationRequest) -> Result<UserAccount, ()> {
        Ok(UserAccount {
            username: "Leon".to_string(),
            avatar: vec![]
        })
    }
}
