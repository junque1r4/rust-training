/*
* Projeto ...: Troca de valores entre variaveis
* Autor .....: Joao Junqueira
* Data ......: 18/10/20
*/

use std::io;
use std::io::prelude::*;

fn main() {
    let mut ler_a = String::new();
    let mut ler_b = String::new();

    let cache: i32;
    let mut a: i32;
    let mut b: i32;
    
    print!("Entre o valor de a ..:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ler_a).unwrap();
    a = ler_a.trim().parse::<i32>().unwrap();

    print!("Entre o valor de b ..:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ler_b).unwrap();
    b = ler_b.trim().parse::<i32>().unwrap();

    print!("A: {}\nB: {}", a, b);


    cache = a;
    a = b;
    b = cache;


    println!();
    print!("Novos valores de A e B, respectivamente ..: {}, {}", a, b); 


    println!();
    print!("Tecle <ENTER> para sair do programa...");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();

}
