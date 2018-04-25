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

    fn find_user_contacts(user_id: i64) -> Result<Vec<UserAccount>, DbError>;
}

impl UserRepo for DatabaseCtx {
    fn find_user_by_auth(request: AuthorisationRequest) -> Result<UserAccount, DbError> {
        let pool = match POOL.lock() {
            Ok(pool) => pool,
            Err(_e) => return Err(DbError::NoDbConnection),
        };

        let users: Vec<UserAccount> = pool.prep_exec("SELECT id, username, avatar, date_created, date_active FROM accounts WHERE username = :username AND password = :password", 
            params! {
                "username" => &request.username,
                "password" => &request.password
            }).map(|result| {
                result.map(Result::unwrap).map(|row| {
                    let (id, username, avatar, date_created, date_active) = mysql::from_row(row);
                    UserAccount {
                        id: id,
                        username: username,
                        avatar: avatar,
                        date_created: date_created,
                        date_active: date_active
                    }
                }).collect() 
            }).unwrap();

        if let Some(user) = users.into_iter().next() {
            Ok(user)
        } else {
            Err(DbError::NotFound)
        }
    }

    fn find_user_contacts(user_id: i64) -> Result<Vec<UserAccount>, DbError> {
        let pool = match POOL.lock() {
            Ok(pool) => pool,
            Err(_e) => return Err(DbError::NoDbConnection),
        };

        let users: Vec<UserAccount> = pool.prep_exec("SELECT c.contact_id as id, u.username, u.avatar, u.date_created, u.date_active FROM accounts_contacts c RIGHT JOIN accounts u ON u.id = c.contact_id WHERE c.user_id = :user_id", 
            params! {
                "user_id" => user_id
            }).map(|result| {
                result.map(Result::unwrap).map(|row| {
                    let (id, username, avatar, date_created, date_active) = mysql::from_row(row);
                    UserAccount {
                        id: id,
                        username: username,
                        avatar: avatar,
                        date_created: date_created,
                        date_active: date_active
                    }
                }).collect() 
            }).unwrap();

        Ok(users)
    }
}