use std::io::{self, Write};         // Importa entrada e saída padrão (para ler e escrever no terminal)
// Import standard I/O (for reading and writing to the terminal)

use std::process::Command;          // Importa para executar comandos do sistema (limpar tela)
// Import to run system commands (used to clear the screen)


// ======================
// MAPA DO LABIRINTO / MAZE MAP
// ======================
// '#' = parede / wall
// ' ' = caminho livre / free path
// 'S' = saída / exit
static LABIRINTO: [[char; 10]; 10] = [
    ['#','#','#','#','#','#','#','#','#','#'],
    ['#',' ',' ',' ',' ','#',' ',' ',' ','#'],
    ['#',' ',' ',' ',' ','#',' ',' ',' ','#'],
    ['#','#','#',' ',' ','#',' ',' ',' ','#'],
    ['#',' ',' ',' ',' ',' ',' ',' ',' ','#'],
    ['#',' ',' ',' ',' ',' ',' ',' ',' ','#'],
    ['#',' ',' ',' ',' ',' ',' ',' ',' ','#'],
    ['#',' ',' ',' ',' ','#',' ',' ',' ','#'],
    ['#',' ',' ',' ',' ','#',' ',' ',' ','#'],
    ['#','#','#','#','#','#','#','#','S','#'],
];


fn main() {
    let mut jo = 1; // linha inicial / starting row
    let mut ju = 1; // coluna inicial / starting column

    // Loop principal do jogo / Main game loop
    loop {
        // Verifica se o jogador chegou à saída / Check if player reached the exit
        if LABIRINTO[jo][ju] == 'S' {
            println!("\x1b[32m🎉 Parabéns, você chegou ao final do labirinto!\x1b[0m");
            // Prints victory message in green
            break;
        }

        limpar_tela();          // Limpa a tela / Clear the screen
        mostrar_labirinto(jo, ju); // Mostra o mapa atualizado / Display the current maze state

        println!("\nUse W (cima), A (esquerda), S (baixo), D (direita) para se mover.");
        println!("Use W (up), A (left), S (down), D (right) to move.");
        print!("Digite seu movimento / Enter your move: ");
        io::stdout().flush().unwrap(); // Garante que o texto apareça antes da leitura / Flushes output buffer

        let mut movimento = String::new();
        io::stdin().read_line(&mut movimento).unwrap(); // Lê entrada do usuário / Reads user input
        let mm = movimento.trim().chars().next().unwrap_or(' '); // Pega a primeira letra / Get first character

        // Define nova posição / Define new position
        let mut nx = jo;
        let mut ny = ju;

        // Interpreta o comando do jogador / Interpret the player's command
        match mm {
            'w' | 'W' => nx = jo.saturating_sub(1), // Move para cima / Move up
            's' | 'S' => nx = jo.saturating_add(1), // Move para baixo / Move down
            'a' | 'A' => ny = ju.saturating_sub(1), // Move para esquerda / Move left
            'd' | 'D' => ny = ju.saturating_add(1), // Move para direita / Move right
            _ => println!("Movimento inválido! / Invalid move!"),
        }

        // Atualiza posição se o movimento for válido / Update position if valid
        if mover(nx, ny) {
            jo = nx;
            ju = ny;
        }
    }
}


// ======================
// MOSTRAR LABIRINTO / DISPLAY MAZE
// ======================
fn mostrar_labirinto(jo: usize, ju: usize) {
    for i in 0..10 {
        for j in 0..10 {
            if i == jo && j == ju {
                // Jogador (E) em azul / Player (E) in blue
                print!("\x1b[34m E\x1b[0m");
            } else {
                // Define cor conforme o tipo de célula / Color according to cell type
                match LABIRINTO[i][j] {
                    '#' => print!("\x1b[31m #\x1b[0m"), // Parede vermelha / Red wall
                    'S' => print!("\x1b[32m S\x1b[0m"), // Saída verde / Green exit
                    _ => print!("  "),                  // Espaço vazio / Empty path
                }
            }
        }
        println!(); // Nova linha / New line
    }
}


// ======================
// MOVIMENTO VÁLIDO? / VALID MOVE?
// ======================
fn mover(x: usize, y: usize) -> bool {
    if x < 10 && y < 10 {
        // Retorna verdadeiro se não for parede / Returns true if not a wall
        LABIRINTO[x][y] != '#'
    } else {
        false // Fora dos limites / Out of bounds
    }
}


// ======================
// LIMPAR TELA / CLEAR SCREEN
// ======================
fn limpar_tela() {
    if cfg!(target_os = "windows") {
        // Comando para Windows / Command for Windows
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        // Comando para Linux/Mac / Command for Linux/Mac
        let _ = Command::new("clear").status();
    }
}
