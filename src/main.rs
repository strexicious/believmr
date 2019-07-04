use std::fs::File;
use std::io::Read;

mod parser;
mod engine;
mod cluster;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // let mut file = File::open(&args[1]).unwrap();
    // let mut code = String::new();
    // file.read_to_string(&mut code).expect("Yo file was not read!");
    // let mut process = engine::Process::new(code);
    // process.run();

    if args[1] == "host" {
        let mut host = cluster::Master::listen_on("127.0.0.1:3000").unwrap();
        host.accept_workers();
    } else if args[1] == "worker" {
        let mut client = cluster::Worker::register(763, "127.0.0.1:3000").unwrap();
    }
}
