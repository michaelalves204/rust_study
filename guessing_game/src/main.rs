use std::io;
//é um enum que representa as diferentes opções de ordenação utilizadas para comparar dois valores, como "menor que", "igual a" ou "maior que"
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng()
                              .gen_range(1..=10);

    println!("Vamos jogar adivinhe o numero ?");

    println!("Por favor informe seu palpite.");

    let mut guess = String::new(); // Mut significa mutavel, ou seja toda variave sem mut é imutavel

    // uso do stdin para output do valor informado pelo usuario
    io::stdin()
        .read_line(&mut guess)
        .expect("Não foi possivel ler a linha");

    // converte e armazena um valor de entrada do usuário, que é uma string, em um número inteiro não assinado de 32 bits (u32)
    let guess: u32 = guess.trim().parse().expect("Precisa ser um numero");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Muito pequeno!"),
      Ordering::Greater => println!("Muito grande!"),
      Ordering::Equal => println!("Você acertou!"),
  }
}
