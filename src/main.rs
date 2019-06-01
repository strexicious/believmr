use std::fs::File;
use std::io::Read;

mod parser;
mod engine;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).expect("Yo file was not read!");
    let mut process = engine::Process::new(code);
    process.run();
}
