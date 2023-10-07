## Iniciando meus estudos com Rust

Rust é uma linguagem de programação de sistema criada pela Mozilla, conhecida por sua ênfase na segurança e desempenho. Ela oferece sistemas de gerenciamento de memória sem garbage collector, garantindo prevenção de erros de acesso e segurança em tempo de compilação. Rust é usada em projetos como o navegador Firefox e é popular para desenvolvimento de software de baixo nível, como sistemas operacionais e programas de alto desempenho.

O Rust disponibiliza um Guia extremamente bem elaborado para orientar os iniciantes na linguagem, abordando os conceitos iniciais: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html.

#### Hello World em Rust!

Meu primeiro codigo em rust foi usando o Cargo (Cargo é o sistema de compilação e o gerenciador de pacotes da Rust)

Para criar o seu projeto com cargo:

```rust
cargo new hello_cargo
```

Por padrão o cargo vai gerar uma pasta src com o arquivo `main.rs` contendo um `println!("Hello, world!");` e um arquivo TOML (Tom’s Obvious, Minimal Formato de , que é o formato de configuração da Cargo.)

Para rodar o hello world é preciso compilarr o projeto e executar

```rust
cargo build
# ou somente
cargo run --release
```
