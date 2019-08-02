use std::fs::File;
use std::io::Read;

mod engine;
mod cluster;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut file = File::open(&args[1]).unwrap();
    let mut source = Vec::new();
    file.read_to_end(&mut source).expect("Error reading source file.");
    
    let mut process = engine::Process::new(source).unwrap();
    process.print_program();
    process.run();
    process.print_mem(0x30, 3);
}
