use std::io::{Result, Error, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::mem;

use super::message::Message;

pub struct Worker {
    id: usize,
    host: TcpStream,
}

impl Worker {
    pub fn register<A: ToSocketAddrs>(id: usize, to: A) -> Result<Self> {
        let mut host = TcpStream::connect(to)?;
        
        let mut buffer = [0; mem::size_of::<Message>()];
        host.read_exact(&mut buffer);
        
        match bincode::deserialize::<Message>(&buffer).unwrap() {
            Message::AskId => {
                host.write_all(&Message::TellId(id).serialized()).unwrap();
                host.flush();
            },
            _ => return Err(Error::new(ErrorKind::Other, "unknown message")),
        }
        println!("Connected to a host!");
        Ok(Self { id, host })
    }
}