use nickel::{Nickel, HttpRouter};

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
        
        configure_routes(&mut server);

        server.get("**", middleware!("test"));

        match server.listen(format!("{}:{}", self.host, self.port)) {
            Ok(_server) => println!("Server started successfully"),
            Err(e) => println!("Failed to start API server, error: {}", e)
        }
    }
}

fn configure_routes(server: &mut Nickel) {
    server.get("/test", middleware! { |req, res|
        "yoo"
    });
}