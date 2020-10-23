extern crate rand;

use std::io;
use std::io::prelude::*;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Adivinhe o numero que pensei!");
    let numero_secreto = rand::thread_rng().gen_range(1, 11);
    
    loop {
        print!("\nEntre um numero..: ");
        io::stdout().flush().unwrap();

        let mut numero_informado = String::new();

        io::stdin().read_line(&mut numero_informado)
            .expect("Erro na entrada de dados!");

        let numero_informado: u32 = match numero_informado.trim().parse(){
            Ok(numero_informado) => numero_informado,
            Err(_) => continue
        };
        println!("Voce informou o valor..: {}", numero_informado);

        match numero_informado.cmp(&numero_secreto) {
            Ordering::Less => println!("O valor e baixo!"),
            Ordering::Greater => println!("O valor e alto!"),
            Ordering::Equal => {
                println!("Voce acertou!");
                break;
            }
        }
    }
    
    println!();
    print!("Tecle <ENTER> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}
