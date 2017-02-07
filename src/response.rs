//response.rs

use std::net;
use rustc_serialize::json;
use std::io::Write;

#[derive(Clone)]
pub struct Response {
    addr: net::SocketAddr,
    numerical_data: Vec<f64>,
    textual_data: String,
    object_data: Vec<u8>,
}

impl Response {
    pub fn send(&self) -> (String, bool) {
        match net::TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                let serializable = SerializableResponse {
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
    num:  Vec<f64>,               //floating-point numerical data
    text: String,                 //text data
    obj: Vec<u8>,                 //serialized encoded object data
}
