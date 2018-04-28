use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult};
use nickel::status::StatusCode::{self, Forbidden, NotFound, BadRequest};
use hyper::method::Method::{Options};
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json::ToJson;

use std::default::Default;
use crypto::sha2::Sha256;

use jwt::{Header, Registered, Token};

use models::{AuthorisationRequest, AuthorisationResponse, UserAccount};
use database::{DatabaseCtx, UserRepo};
use error::{AuthorisationError, ErrorString};

lazy_static! {
    static ref AUTH_SECRET: &'static str = "*GDF_LCkE=Aa,G:RQ6CHQXKt{@X/E#)e84N#rk+YNNC7j0mtOipWS#[igFg|ikj";
}

pub struct SafehouseApi {
    host: &'static str,
    port: i16
}

impl SafehouseApi {
    pub fn new(host: &'static str, port: i16) -> Self {
        Self {
            host: host,
            port: port
        }
    }
 
    pub fn start(&self) {
        let mut server = Nickel::new();

        self.init_routes(&mut server);

        match server.listen(format!("{}:{}", self.host, self.port)) {
            Ok(_server) => println!("Server started successfully"),
            Err(e) => println!("Failed to start API server, error: {}", e)
        }
    }

    pub fn init_routes(&self, server: &mut Nickel) {
        server.utilize(authorization_check);
        
        server.post("/authorise", middleware! { |req, mut res|
            let info = req.json_as::<AuthorisationRequest>().unwrap();

            if let Ok(user) = DatabaseCtx::find_user_by_auth(info) {
                let data = user.clone();

                AuthorisationResponse {
                    token: generate_token("Safehouse-Server", data.id),
                    data: data
                }.to_json()
            } else {
                res.set(NotFound);
                format!("Failed to find user").to_json()
            }
        });

        server.get("/contacts", middleware! { | req, res| 
            if let Ok(user_id) = validate_token(&req) {
                if let Ok(contacts) = DatabaseCtx::find_user_contacts(user_id) { // soon we get the id from the token
                    contacts.to_json()
                } else {
                    vec![0].to_json()
                }
            } else {
                vec![0].to_json()
            }
        });
    }
}

fn validate_token(req: &Request) -> Result<i64, AuthorisationError> {
     match req.origin.headers.get::<Authorization<Bearer>>() {
        Some(header) => {
            let token = Token::<Header, Registered>::parse(&header.token).unwrap();
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
        },

        None => Err(AuthorisationError::DataMissing)
    }
}

fn generate_token(issuer: &'static str, user_id: i64) -> String {
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

fn authorization_check<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    if req.origin.method == Options {
        res.next_middleware()
    } else {
        match req.origin.uri.to_string().as_ref() {
            "/authorise" => res.next_middleware(),
            "/status" => res.next_middleware(),

            _ => match validate_token(&req) {
                Ok(_) => res.next_middleware(),
                Err(err) => res.error(Forbidden, err.get_string())
            }
        }
    }
}
