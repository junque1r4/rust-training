/*
* Projeto ...: Conversor de Fahrenheit para Celcius
* Autor .....: Joao Junqueira 
* Data ......: 18/10/20
*/

use std::io;
use std::io::prelude::*;

fn main() {
    let mut ler_fah = String::new();

    let fahrenheit: f64;
    let celcius: f64;

    print!("Digite a temperatura em Fahrenheit ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ler_fah).unwrap();
    fahrenheit = ler_fah.trim().parse::<f64>().unwrap();

    celcius = ((fahrenheit - 32.) * 5.) / 9.;
    print!("O valor convertido sera ..: {:3.2}o", celcius);

    println!();
    print!("Digite <ENTER> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}
