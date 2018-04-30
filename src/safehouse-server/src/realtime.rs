use std::thread;
use std::io::BufReader;
use std::sync::Mutex;
use std::collections::HashMap;
use ws::{listen, Result, Sender, Message, CloseCode, Handler};
use bytebuffer::{ByteBuffer};
use auth::verify_token;
use database::{DatabaseCtx, UserRepo};
use models::{UserStatus, ContactStatus};

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

    pub fn get_status(user_id: i64) -> UserStatus {
        let clients = match REALTIME_CLIENTS.lock() {
            Ok(clients) => clients,
            Err(_e) => return UserStatus::Offline,
        };

        if clients.contains_key(&user_id) {
            UserStatus::Online
        } else {
            UserStatus::Offline
        }
    }
}

struct WebSocket {
    socket: Sender,
    user_id: Option<i64>
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
            RealtimeEvent::Authenticate(token) => handle_authentication(self, token),
            RealtimeEvent::GetStatus() => handle_get_status(self),

            _ => {
                println!("Unhandled request");
            }
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        if let Some(user_id) = self.user_id {
            let mut clients = match REALTIME_CLIENTS.lock() {
                Ok(clients) => clients,
                Err(_e) => return
            };

            clients.remove(&user_id);
        }

        println!("WebSocket closing for ({:?}) {}", code, reason);
    }
}

fn handle_authentication(client: &mut WebSocket, token: String) {
    let user_id = match verify_token(&token) {
        Ok(user_id) => user_id,
        Err(_) => return
    };

    client.user_id = Some(user_id)
}

fn handle_get_status(client: &mut WebSocket) {
    let user_id = match client.user_id {
        Some(user_id) => user_id,
        None => return
    };

    let contacts = match DatabaseCtx::find_user_contacts(user_id) {
        Ok(contacts) => contacts,
        Err(_) => return
    };

    let mut status_vec = Vec::new();

    for contact in contacts {
        status_vec.push(ContactStatus {
            id: contact.id,
            status: SafehouseRealtime::get_status(contact.id)
        })
    }

    println!("{:?}", status_vec);
}

fn start_realtime(host: &'static str, port: i16) {
    thread::spawn(move || {
        println!("Starting realtime server on address: ws://{}:{}", host, port);
        
        listen(format!("{}:{}", host, port), |out| {
            println!("ws connected");
            WebSocket { socket: out, user_id: None }
        });
    });
}