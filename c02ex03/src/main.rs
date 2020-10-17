use std::io;
use std::io::prelude::*;

fn main() {
    let mut valor = String::new();
    let vlr: i64;

    print!("Send a integer number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();
    vlr = valor.trim().parse::<i64>().unwrap();

    println!();
    println!("Result = {}", vlr);

    println!();
    print!("Press <ENTER> to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
