use std::io;
use std::io::prelude::*;

fn main() {
    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let a: u8;
    let b: u8;
    let r: u8;

    print!("Entre o valor <A> .: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<u8>().unwrap();

    
    print!("Entre o valor <B> .: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<u8>().unwrap();
    r = a + b;

    println!();

    if r > 10 {
        println!("Resultado ...: {}", r);
    }

    println!();
    print!("Tecle <ENTER> para sair.");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}
