# 🦀 Meu-Rust-start

Repositório de aprendizado em Rust com foco em sistemas de baixo nível. Rust é uma linguagem de nicho no Brasil — especialmente fora dos grandes centros — e esta é minha aposta para o futuro.

> *“Rust é o que C deveria ter sido desde o início.”*

-----

## 🗂️ Projetos

### 🧮 Calculator (`calculator.rs`)

Calculadora de linha de comando com operações aritméticas básicas.

**O que pratica:**

- Funções tipadas com `f32`
- Leitura de entrada do usuário com `io::stdin`
- Pattern matching com `match`
- Ownership e borrowing na prática
- Conversão e tratamento de tipos

```bash
rustc calculator.rs -o calculator
./calculator
```

-----

### 🌀 Game Labirinto (`Game_labirint.rs`)

Jogo de labirinto funcional rodando no terminal com navegação em tempo real.

**Funcionalidades:**

- Mapa 10x10 com paredes, caminhos e saída
- Movimentação via teclado (WASD)
- Renderização colorida no terminal com códigos ANSI
- Detecção de colisão com paredes
- Detecção de vitória ao chegar na saída `S`
- Compatível com Linux e Windows

**O que pratica:**

- Arrays estáticos multidimensionais (`static`)
- Loop de jogo (`loop`)
- Pattern matching com múltiplos casos
- Manipulação de `usize` e `saturating_sub/add` para evitar overflow
- Comandos do sistema com `std::process::Command`
- Flush de buffer de saída com `io::Write`

```bash
rustc Game_labirint.rs -o labirinto
./labirinto
```

-----

## 🧠 Por que Rust?

- **Segurança de memória sem garbage collector** — o compilador garante que não há dangling pointers ou data races
- **Performance equivalente a C/C++** — sem overhead de runtime
- **Adoção crescente no kernel Linux** — desde 2022, Rust é linguagem oficial do kernel
- **Demanda no mercado** — empresas como Microsoft, Google, Amazon e Mozilla usam Rust em sistemas críticos
- **Nicho no Brasil** — pouquíssimos desenvolvedores brasileiros dominam Rust, o que cria uma vantagem real

-----

## 📖 O que estou aprendendo

- [x] Ownership e borrowing
- [x] Pattern matching com `match`
- [x] Tipos primitivos e conversão
- [x] Entrada e saída no terminal
- [x] Arrays estáticos e indexação segura
- [ ] Structs e implementações (`impl`)
- [ ] Enums e tipos customizados
- [ ] Tratamento de erros com `Result` e `Option`
- [ ] Gerenciamento de memória com `Box`, `Rc`, `Arc`
- [ ] Traits e generics
- [ ] Unsafe Rust

-----

## 🚀 Próximos projetos planejados

- [ ] Estruturas de dados (lista encadeada, pilha, fila) em Rust puro
- [ ] Alocador de memória simples usando `unsafe`
- [ ] Shell básico em Rust
- [ ] Reescrita do jogo labirinto com struct `Jogador` e mapa dinâmico

-----

## 🔗 Outros repositórios

- [Estruturas-de-dados em C](https://github.com/10daniel2001/Estruturas-de-dados-) — Base teórica em C para entender o que Rust abstrai

-----

## 👤 Autor

**Daniel** — Estudante de Engenharia de Software (3º semestre)  
Foco em sistemas de baixo nível, Rust e entender como o SO funciona de verdade.

[![GitHub](https://img.shields.io/badge/GitHub-10daniel2001-black?logo=github)](https://github.com/10daniel2001)