use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult};
use nickel::status::StatusCode::{self, Forbidden, NotFound, BadRequest};
use hyper::method::Method::{Options};
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json::ToJson;

use models::{AuthorisationRequest};
use database::{DatabaseCtx, UserRepo};

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

        server.get("**", middleware!("test"));

        server.post("/authorize", middleware! { |req, mut res|
            let info = try_with!(res, {
                req.json_as::<AuthorisationRequest>().map_err(|e| (BadRequest, e))
            });

            if let Ok(user) = DatabaseCtx::find_user_by_auth(info) {
                user.to_json()
            } else {
                res.set(NotFound);

                format!("Failed to find user").to_json()
            }
        });

        server.get("/", middleware! { |req, res |
            "hi"
        });
    }
}

fn validate_token(token: &str) -> Result<(), &'static str> {
    match token {
        "lol" => Ok(()),
        _ => Err("Invalid token")
    }
}

fn authorization_check<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    if req.origin.method == Options {
        res.next_middleware()
    } else {
        match req.origin.uri.to_string().as_ref() {
            "/authorize" => res.next_middleware(),
            "/status" => res.next_middleware(),

            _ => match req.origin.headers.get::<Authorization<Bearer>>() {
                Some(header) => match validate_token(&header.token) {
                    Ok(_) => res.next_middleware(),
                    Err(err) => res.error(Forbidden, err)
                },
                
                None => res.error(Forbidden, "No token set")
            }
        }
    }
}
