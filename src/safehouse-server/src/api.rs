
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult};
use nickel::status::StatusCode::{self, Forbidden};
use hyper::method::Method::{Options};

use hyper::header::{Authorization, Bearer};

pub struct SafehouseApi {
    host: &'static str,
    port: i16
}

impl SafehouseApi {
    pub fn new(host: &'static str, port: i16) -> SafehouseApi {
        SafehouseApi {
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

        server.post("/authorize", middleware! { |req, res |
            let info = try_with!(res, {
                req.json_as::<AuthorisationRequest>().map_err(|e| (StatusCode::BadRequest, e))
            });

            println!("username: {}, password {}", info.username, info.password);
            format!("hi {}", info.username)
        });

        server.get("/", middleware! { |req, res |
            "hi"
        });
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct AuthorisationRequest {
    username: String,
    password: String
}

fn validate_token(token: &str) -> Result<(), &'static str> {
    match token {
        "lol" => Ok(()),
        _ => Err("Invalid token")
    }
}

fn authorization_check<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    match req.origin.uri.to_string().as_ref() {
        "/authorize" => res.next_middleware(),

        _ => {
            match &req.origin.method {
                Options => res.next_middleware(),

                _ => {
                    match req.origin.headers.get::<Authorization<Bearer>>() {
                        Some(header) => match validate_token(&header.token) {
                            Ok(_) => res.next_middleware(),
                            Err(err) => res.error(Forbidden, err)
                        },

                        None => res.error(Forbidden, "No token set")
                    }
                }
            }
        }
    }
}