use std::fs::File;
use std::io::Read;

mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    for line in code.lines() {
        println!("{:?}", parser::instr().parse(line.as_bytes()).unwrap());
    }
}
