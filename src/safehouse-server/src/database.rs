use mysql;

use models::{AuthorisationRequest, UserAccount};

use error::DbError;

use std::env;
use std::sync::{Mutex, Arc};

lazy_static! {
    static ref POOL: Mutex<mysql::Pool> = Mutex::new(mysql::Pool::new({
            // fetch from another source, e.g env vars
            "mysql://root:password@localhost:3306/safehouse"
        }).unwrap());
}


#[derive(Debug)]
pub struct DatabaseCtx {}

pub trait UserRepo {
    fn find_user_by_auth(request: AuthorisationRequest) -> Result<UserAccount, DbError>;
}

impl UserRepo for DatabaseCtx {
    fn find_user_by_auth(request: AuthorisationRequest) -> Result<UserAccount, DbError> {
        let pool = match POOL.lock() {
            Ok(pool) => pool,
            Err(_e) => return Err(DbError::NoDbConnection),
        };

        let users: Vec<UserAccount> = pool.prep_exec("SELECT id, username, avatar FROM accounts WHERE username = :username AND password = :password", params! {
                "username" => &request.username,
                "password" => &request.password
            }).map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, username, avatar) = mysql::from_row(row);
                    UserAccount {
                        id: id,
                        username: username,
                        avatar: avatar,
                    }
                }).collect() 
            }).unwrap();

        if let Some(user) = users.first() {
            Ok(UserAccount {
                id: user.id,
                username: user.username.clone(),
                avatar: user.avatar.clone()
            })
        } else {
            Err(DbError::NotFound)
        }
    }
}