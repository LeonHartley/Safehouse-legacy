use std::thread;
use std::io::BufReader;
use std::sync::Mutex;
use std::collections::HashMap;
use ws::{listen, Result, Sender, Message, CloseCode, Handler};
use bytebuffer::{ByteBuffer};

lazy_static! {
    static ref REALTIME_CLIENTS: Mutex<HashMap<i64, WebSocket>> = Mutex::new(HashMap::new());
}

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

struct WebSocket {
    socket: Sender,
    token: Option<String>
}

pub enum RealtimeEvent {
    Authenticate(String),
    GetStatus(),
    Unknown()
}

impl RealtimeEvent {
    fn parse(client: &WebSocket, mut data: Vec<u8>) -> RealtimeEvent {
        let mut buffer = ByteBuffer::from_bytes(&mut data);
        
        let id = buffer.read_u16();
        let payload_len = buffer.read_u16() as usize;
        let payload = buffer.read_bytes(payload_len);

        println!("Id: {}, Payload len: {}, Vec len: {}", id, payload_len, data.len());

        match id {
            1 => RealtimeEvent::Authenticate(String::from_utf8(payload).unwrap()),
            2 => RealtimeEvent::GetStatus(),
            _ => RealtimeEvent::Unknown()
        }
    }
}

impl Handler for WebSocket {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg);

        match RealtimeEvent::parse(&self, msg.into_data()) {
            RealtimeEvent::Authenticate(token) => {
                println!("Requesting authentication, token: {}", token);
            }
            
            _ => {
                println!("requested somethin else");
            }
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
    }
}

fn start_realtime(host: &'static str, port: i16) {
    thread::spawn(move || {
        println!("Starting realtime server on address: ws://{}:{}", host, port);
        
        listen(format!("{}:{}", host, port), |out| {
            println!("ws connected");
            WebSocket { socket: out, token: None }
        });
    });
}