use std::thread;
use ws::{listen};

pub struct SafehouseRealtime {
    host: &'static str,
    port: i16
}

impl SafehouseRealtime {
    pub fn new(host: &'static str, port: i16) -> SafehouseRealtime {
        SafehouseRealtime {
            host: host, 
            port: port
        }
    }

    pub fn start(&self) {
        start_realtime(self.host, self.port)
    }
}

fn start_realtime(host: &'static str, port: i16) {
    thread::spawn(move || {
        println!("Starting realtime server on address: ws://{}:{}", host, port);
        
        listen(format!("{}:{}", host, port), |out| {
            move |msg| {
                out.send(msg)
            }
        });
    });
}