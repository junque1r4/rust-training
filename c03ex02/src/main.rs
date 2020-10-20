/*
* Projeto ...: Troca de valores entre variaveis
* Autor .....: Joao Junqueira
* Data ......: 19/10/20
*/

use std::io;
use std::io::prelude::*;


fn main() {
    let mut valor_a = String::new();
    let mut valor_b = String::new();

    let a: i32;
    let b: i32;
    let r: i32;

    print!("Entre o valor <A> .: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_a).unwrap();
    a = valor_a.trim().parse::<i32>().unwrap();

    
    print!("Entre o valor <B> .: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_b).unwrap();
    b = valor_b.trim().parse::<i32>().unwrap();
    r = a * b;

    println!();

    if r >= 20 {
        println!("Resultado ..: {}", r + 5);
    } else {
        println!("Resultado ..: {}", r - 7);
    }

    println!();
    print!("Tecle <ENTER> para sair.");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}
