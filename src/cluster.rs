use std::io::{Result, Error, ErrorKind, Read, Write};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct Worker {
    id: usize,
    host: TcpStream,
}

impl Worker {
    pub fn register<A: ToSocketAddrs>(id: usize, to: A) -> Result<Self> {
        let mut host = TcpStream::connect(to)?;
        let mut msg = [0; 512];
        let bytes_read = host.read(&mut msg).unwrap();
        match std::str::from_utf8(&msg[..bytes_read]).unwrap() {
            "tell id" => {
                host.write_all(&id.to_ne_bytes()).unwrap();
                host.flush();
            },
            _ => return Err(Error::new(ErrorKind::Other, "unknown message")),
        }
        println!("Connected to a host!");
        Ok(Self { id, host })
    }
}

pub struct Host {
    workers: HashMap<usize, TcpStream>,
    listener: TcpListener,
}

impl Host {
    pub fn listen_on<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Self { workers: HashMap::new(), listener: TcpListener::bind(addr)? })
    }

    pub fn accept_workers(&mut self) {
        for worker in self.listener.incoming() {
            match worker {
                Ok(mut worker) => {
                    worker.write_all("tell id".as_bytes());
                    worker.flush();
                    let mut id = [0; 8];
                    worker.read_exact(&mut id).unwrap();
                    let id = usize::from_ne_bytes(id);
                    self.workers.insert(id, worker);
                    println!("A new worker was added: {}", id);
                },
                Err(e) => println!("Worker failed to connect: {}", e)
            }
        }
    }
}
