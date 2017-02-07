//response.rs

use std::net;
use rustc_serialize::json;
use std::io::Write;

pub const CODE_TOKEN_ACKNOWLEDGE: u8 = 0;
pub const CODE_CONNECT_ACCEPT: u8 = 1;
pub const CODE_CONNECT_REJECT: u8 = 2;
pub const CODE_DISCONNECT_ACK: u8 = 3;

#[derive(Clone)]
pub struct Response {
    pub code: u8,
    pub addr: net::SocketAddr,
    pub numerical_data: Vec<f64>,
    pub textual_data: String,
    pub object_data: Vec<u8>,
}

impl Response {
    pub fn send(&self) -> (String, bool) {
        match net::TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                let serializable = SerializableResponse {
                    code: self.code,
                    num: self.numerical_data.clone(),
                    text: self.textual_data.clone(),
                    obj: self.object_data.clone(),
                };
                let encoded = json::encode(&serializable).unwrap();
                let _ = stream.write_all(encoded.as_bytes());
                (format!("Sent response to {}", self.addr), true)
            },
            Err(why) => (format!("Failed to open connection for sending response to {}: {}", self.addr, why), false),
        }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct SerializableResponse {
    code: u8,
    num:  Vec<f64>,               //floating-point numerical data
    text: String,                 //text data
    obj: Vec<u8>,                 //serialized encoded object data
}
