use std::fs::File;
use std::io::Read;

mod engine;
mod cluster;

use engine::SysCallInvoc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut file = File::open(&args[1]).unwrap();
    let mut source = Vec::new();
    file.read_to_end(&mut source).expect("Error reading source file.");
    
    fn handle_syscall(syscall_invoc: &SysCallInvoc) {
        use SysCallInvoc::*;
        
        match syscall_invoc {
            Print(s) => print!("{}", s),
        }
    }
    
    println!("{:?}", source);
    let mut process = engine::Process::new(source, handle_syscall).unwrap();
    process.run();
}
