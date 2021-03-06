use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult};
use nickel::status::StatusCode::{self, Forbidden, NotFound, BadRequest};
use hyper::method::Method::{Options};
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowHeaders, Authorization, Bearer};
use rustc_serialize::json::ToJson;

use models::{AuthorisationRequest, AuthorisationResponse, UserAccount};
use database::{DatabaseCtx, UserRepo};
use error::{AuthorisationError, ErrorString};
use auth::{verify_token, generate_token};

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

    fn init_routes(&self, server: &mut Nickel) {
        server.utilize(authorization_check);
        server.utilize(enable_cors);

        server.options("**", middleware!(""));

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
            let user_id = match validate_token(&req) {
                Ok(user_id) => user_id,
                Err(err) => return res.error(Forbidden, err.get_string())
            };

            if let Ok(contacts) = DatabaseCtx::find_user_contacts(user_id) {
                contacts.to_json()
            } else {
                vec![0].to_json()
            }
        });
    }
}

fn validate_token(req: &Request) -> Result<i64, AuthorisationError> {
    match req.origin.headers.get::<Authorization<Bearer>>() {
        Some(header) => {
          verify_token(&header.token)
        },

        None => Err(AuthorisationError::DataMissing)
    }
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

fn enable_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.set(AccessControlAllowOrigin::Any);
    res.set(AccessControlAllowHeaders(vec![
        "Origin".into(),
        "X-Requested-With".into(),
        "Content-Type".into(),
        "Accept".into(),
        "Authorization".into()
    ]));

    res.next_middleware()
}

