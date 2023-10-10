fn main() {
    // String
    let greeting: &str = "Hello, Rust!"; // Referência a uma string literal
    let x: i32 = 42; // Inteiro com sinal de 32 bits
    let y: u64 = 123456; // Inteiro sem sinal de 64 bits
    // Rust suporta números de ponto flutuante de precisão única (f32) e dupla precisão (f64):
    let a: f32 = 3.14;
    let b: f64 = 2.71828;
    // Boolean
    let is_rust_cool: bool = true;
    let is_python_cool: bool = false;
    // Caracteres em Rust são representados como char, que suporta caracteres Unicode:
    let letter: char = 'A';
    let emoji: char = '😀';
    // Coleções
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let first: i32 =  numbers[0];
    // Tuplas
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_s, d, _f) = tup;

    println!("Data type: String {}", greeting);
    println!("Data type: Integer {}, {}", x, y);
    println!("Data type: Float {}, {}", a, b);
    println!("Data type: Boolean {} {}", is_rust_cool, is_python_cool);
    println!("Data type: Char {} {}", letter, emoji);
    println!("Data type: Array {:?} {}", numbers, first);
    println!("Data type: Tuple {:?} {}", tup, d)
}

//Integer
// Comprimento Assinado Não assinado
// 8 bits	   i8	    u8
// 16-bit	   i16	    u16
// 32-bit	   i32	    u32
// 64-bit	   i64	    u64
// 128 bits	   i128	    u128