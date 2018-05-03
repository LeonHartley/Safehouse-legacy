use std::thread;
use std::io::BufReader;
use std::sync::{Mutex};
use std::collections::HashMap;
use ws::{listen, Result, Sender, Message, CloseCode, Handler};
use bytebuffer::{ByteBuffer};
use auth::verify_token;
use database::{DatabaseCtx, UserRepo};
use models::{UserStatus, ContactStatus, ChatMessage};
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

    pub fn get_status(user_id: i64, clients: &HashMap<i64, Sender>) -> UserStatus {
        if clients.contains_key(&user_id) {
            UserStatus::Online
        } else {
            UserStatus::Offline
        }
    }

    pub fn send_msg(user_id: &i64, msg_type: u16, payload: String, clients: &HashMap<i64, Sender>) {
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
    SendMessage(ChatMessage),
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
            3 => RealtimeEvent::SendMessage(json::decode(&String::from_utf8(payload).unwrap()).unwrap()),
            _ => RealtimeEvent::Unknown()
        }
    }
}

impl Handler for WebSocket {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        match RealtimeEvent::parse(&self, msg.into_data()) {
            RealtimeEvent::Authenticate(token) => handle_authentication(self, token),
            RealtimeEvent::GetStatus() => handle_get_status(self),
            RealtimeEvent::SendMessage(message) => handle_send_message(self, message),

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
            self.notify_status(UserStatus::Offline, &clients);
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
    fn notify_status(&self, status: UserStatus, clients: &HashMap<i64, Sender>) {
        if let Some(user_id) = self.user_id {
            if let Ok(contacts) = self.contacts.as_ref().unwrap().lock() {
                let msg = json::encode(&ContactStatus {
                    id: user_id,
                    status
                }).unwrap();

                for c in contacts.iter() {
                    SafehouseRealtime::send_msg(c, 2, msg.clone(), &clients);
                };
            }
        }
    }
}

fn handle_authentication(client: &mut WebSocket, token: String) {
    if let Ok(user_id) = verify_token(&token) {
        let contact_data = match DatabaseCtx::find_user_contacts(user_id) {
            Ok(contact_data) => contact_data,
            Err(_) => return
        };

        let mut contacts = Vec::new();

        for contact in contact_data {
            contacts.push(contact.id);
        }

        client.user_id = Some(user_id);
        client.contacts = Some(Mutex::new(contacts));

        if let Ok(mut clients) = REALTIME_CLIENTS.lock() {
            clients.insert(user_id, client.socket.clone());
            client.notify_status(UserStatus::Online, &clients);
        }
    }
}

fn handle_get_status(client: &WebSocket) {
    if let Some(user_id) = client.user_id {
        if let Ok(contacts) = client.contacts.as_ref().unwrap().lock() {
            let clients = match REALTIME_CLIENTS.lock() {
                Ok(clients) => clients,
                Err(_e) => return
            };

            let mut status_vec = vec![];
            
            for contact in contacts.iter() {
                status_vec.push(ContactStatus {
                    id: *contact,
                    status: SafehouseRealtime::get_status(*contact, &clients)
                })
            };

            client.socket.send_msg(2, json::encode(&status_vec).unwrap());
        }
    }
}

fn handle_send_message(client: &WebSocket, message: ChatMessage) {
    if let Some(user_id) = client.user_id {
        if let Ok(contacts) = client.contacts.as_ref().unwrap().lock() {
            let clients = match REALTIME_CLIENTS.lock() {
                Ok(clients) => clients,
                Err(_e) => return
            };

            if contacts.contains(&message.user_id) {
                println!("Message sent: {:?}", message);
                SafehouseRealtime::send_msg(&message.user_id, 3, json::encode(&message).unwrap(), &clients)
            }
        }
    }
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