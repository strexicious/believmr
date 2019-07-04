use std::io::{Result, Error, ErrorKind, Read, Write};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::mem;

use super::message::{Message, MESSAGE_SIZE};

pub struct Master {
    workers: HashMap<usize, TcpStream>,
    listener: TcpListener,
}

impl Master {
    pub fn listen_on<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Self { workers: HashMap::new(), listener: TcpListener::bind(addr)? })
    }

    pub fn accept_workers(&mut self) {
        for worker in self.listener.incoming() {
            match worker {
                Ok(mut worker) => {
                    worker.write_all(&Message::AskId.serialized()).unwrap();
                    worker.flush();

                    let mut buffer = [0; MESSAGE_SIZE];
                    worker.read_exact(&mut buffer);
                    if let Message::TellId(id) = bincode::deserialize(&buffer).unwrap() {
                        self.workers.insert(id, worker);
                        println!("A new worker was added: {}", id);
                    }
                },
                Err(e) => println!("Worker failed to connect: {}", e)
            }
        }
    }
}