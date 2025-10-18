use std::io; 
// EN: Import the input/output library from the Rust standard library.
// PT: Importa a biblioteca de entrada/saída da biblioteca padrão do Rust.

// --- Basic arithmetic functions ---
// --- Funções aritméticas básicas ---

fn somar(a: f32, b: f32) -> f32 { a + b }       
// EN: Function that returns the sum of two floating-point numbers.
// PT: Função que retorna a soma de dois números de ponto flutuante.

fn multiplicar(a: f32, b: f32) -> f32 { a * b }  
// EN: Function that returns the product of two floating-point numbers.
// PT: Função que retorna o produto de dois números de ponto flutuante.

fn dividir(a: f32, b: f32) -> f32 { a / b }      
// EN: Function that returns the division result of two floating-point numbers.
// PT: Função que retorna o resultado da divisão entre dois números de ponto flutuante.


// --- Main Function ---
// --- Função Principal ---

fn main() {
    // EN: Ask the user for a math operator.
    // PT: Pede ao usuário um operador matemático.
    println!("Digite um operador * + /");

    let mut operador = String::new();
    io::stdin().read_line(&mut operador).expect("Erro de texto");
    let operador = operador.trim(); 
    // EN: Removes extra spaces and newline from the input.
    // PT: Remove espaços extras e quebra de linha da entrada.

    // EN: Ask for the first number.
    // PT: Pede o primeiro número.
    println!("Digite o primeiro numero");
    let mut numero1 = String::new();
    io::stdin().read_line(&mut numero1).expect("Erro");
    let numero1: f32 = numero1.trim().parse().expect("Erro de numero");
    // EN: Converts the input from string to a floating-point number.
    // PT: Converte a entrada de string para número de ponto flutuante.

    // EN: Ask for the second number.
    // PT: Pede o segundo número.
    println!("Digite o segundo numero");
    let mut numero2 = String::new();
    io::stdin().read_line(&mut numero2).expect("Erro");
    let numero2: f32 = numero2.trim().parse().expect("Erro de numero");
    // EN: Converts the second input to f32 as well.
    // PT: Converte a segunda entrada também para f32.

    // --- Operator handling ---
    // --- Tratamento do operador ---
    match operador {
        "*" => println!("O resultado é {}", multiplicar(numero1, numero2)),
        "+" => println!("O resultado é {}", somar(numero1, numero2)),
        "/" => println!("O resultado é {}", dividir(numero1, numero2)),
        _ => println!("Erro: operador inválido"),
    }
    // EN: Checks which operator was entered and calls the corresponding function.
    // PT: Verifica qual operador foi digitado e chama a função correspondente.
}
