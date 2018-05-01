use std::thread;
use std::io::BufReader;
use std::sync::{Mutex};
use std::collections::HashMap;
use ws::{listen, Result, Sender, Message, CloseCode, Handler};
use bytebuffer::{ByteBuffer};
use auth::verify_token;
use database::{DatabaseCtx, UserRepo};
use models::{UserStatus, ContactStatus};
use rustc_serialize::{json};

lazy_static! {
    static ref REALTIME_CLIENTS: Mutex<HashMap<i64, Sender>> = Mutex::new(HashMap::new());
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

    pub fn send_msg(user_id: &i64, msg_type: u16, payload: String) {
        let clients = match REALTIME_CLIENTS.lock() {
            Ok(clients) => clients,
            Err(e) => return
        };

        if let Some(client) = clients.get(&user_id) {
            client.send_msg(msg_type, payload);
        }
    }
}

struct WebSocket {
    socket: Sender,
    user_id: Option<i64>,
    contacts: Option<Mutex<Vec<i64>>>
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

        match id {
            1 => RealtimeEvent::Authenticate(String::from_utf8(payload).unwrap()),
            2 => RealtimeEvent::GetStatus(),
            _ => RealtimeEvent::Unknown()
        }
    }
}

impl Handler for WebSocket {
    fn on_message(&mut self, msg: Message) -> Result<()> {
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

trait SendMessage {
    fn send_msg(&self, msg_type: u16, payload: String);
}

impl SendMessage for Sender {
    fn send_msg(&self, msg_type: u16, payload: String) {
        let mut buffer = ByteBuffer::new();

        buffer.write_u16(msg_type);
        buffer.write_u16(payload.len() as u16);
        buffer.write_bytes(&payload.into_bytes());

        self.send(buffer.to_bytes());
    }
}

impl WebSocket {
    fn notify_status(&self, status: UserStatus) {
        let contacts = match self.contacts {
            Some(ref contacts) => match contacts.lock() {
                Ok(contacts) => contacts,
                Err(_) => return
            },

            None => return
        };

        let user_id = match self.user_id {
            Some(user_id) => user_id,
            None => return
        };

        let contact_status = ContactStatus {
            id: user_id,
            status
        };

        for c in contacts.iter() {
            SafehouseRealtime::send_msg(c, 2, json::encode(&contact_status).unwrap());
        };
    }
}

fn handle_authentication(client: &mut WebSocket, token: String) {
    let user_id = match verify_token(&token) {
        Ok(user_id) => user_id,
        Err(_) => return
    };

    client.user_id = Some(user_id);

    let contact_data = match DatabaseCtx::find_user_contacts(user_id) {
        Ok(contact_data) => contact_data,
        Err(_) => return
    };

    let mut contacts = Vec::new();

    for contact in contact_data {
        contacts.push(contact.id);
    }

    client.contacts = Some(Mutex::new(contacts));

    if let Ok(mut clients) = REALTIME_CLIENTS.lock() {
        clients.insert(user_id, client.socket.clone());
    };

    client.notify_status(UserStatus::Online);

}

fn handle_get_status(client: &WebSocket) {
    let user_id = match client.user_id {
        Some(user_id) => user_id,
        None => return
    };

    //client.socket.send_msg(2, json::encode(&status_vec).unwrap());
}

fn start_realtime(host: &'static str, port: i16) {
    thread::spawn(move || {
        println!("Starting realtime server on address: ws://{}:{}", host, port);
        
        listen(format!("{}:{}", host, port), |out| {
            println!("ws connected");
            WebSocket { socket: out, user_id: None, contacts: None }
        });
    });
}