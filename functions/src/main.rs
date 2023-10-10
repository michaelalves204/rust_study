fn main() {
    another_function();
    another_function_parameter(25);
    print_labeled_measurement(20, 'M');

    // Declarações são instruções que executam alguma ação e não retornam um valor.
    // As expressões avaliam um valor resultante.
    let x = five();

    let y = {
        let x = 3;
        x + 1
    };

    let plus_one :i32 = plus_one(5);

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of plus_one is: {}", plus_one);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_parameter(x: i32) {
    println!("The value of x is {}", x);
}
// Nas assinaturas de função, você deve declarar o tipo de cada parâmetro
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label);
}
// As funções podem retornar valores ao código que os chama
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
