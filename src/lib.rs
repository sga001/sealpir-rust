extern crate libc;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Clone)]
pub struct PirQuery {
    pub query: Vec<u8>,
    pub num: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PirReply {
    pub reply: Vec<u8>,
    pub num: u32,
}

pub mod client;
pub mod server;
