use std::io;
use std::io::prelude::*;

fn main() {
    let mut nome = String::new();

    print!("Write your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nome).expect("Wrong Input");

    if let Some('\n') = nome.chars().next_back() {
        nome.pop();
    }

    if let Some('r') = nome.chars().next_back() {
        nome.pop();
    }
    
    println!("Hello, {}.\n", nome);
    print!("Press <ENTER> to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
