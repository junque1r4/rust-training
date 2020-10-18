/*
* Projeto ...: Conversor de Celcius para fahrenheit
* Autor .....: Joao Junqueira
* Data ......: 18/10/20
*/

use std::io;
use std::io::prelude::*;

fn main() {
    let mut ler_celc = String::new();
    
    let celcius: f64;
    let fahrenheint: f64;
    
    print!("Digite a temperatura em Celcius ......: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ler_celc).unwrap();
    celcius = ler_celc.trim().parse::<f64>().unwrap();

    fahrenheint = (9. * celcius + 160.) / 5.;


    print!("A temperatura de {:3.2}f representa ...: {:3.2} celcius", celcius, fahrenheint);
    println!();
    print!("Tecle <ENTER> para sair.");
    io::stdout().flush().unwrap();
    io::stdin().read_exact(&mut [0u8]).unwrap();
}
