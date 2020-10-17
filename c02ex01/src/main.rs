use std::io;
use std::io::prelude::*;

fn main() {
    println!("Programming in Rust.\n");
    print!("Press <ENTER> to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
