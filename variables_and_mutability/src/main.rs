// como variáveis imutáveis,
// constantes são valores que estão vinculados
// a um nome e Não é permitido mudar
// não é possivel usar mut com constantes.
// elas são sempre imutáveis.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    //por padrão toda variavel é imutavel
    //para que mossamos manipular os valores é preciso adicionar mut
    let mut x = 5;
    println!("o valor de x é: {x}");

    x = 6;
    println!("o valor de x é: {x}");

    println!("o valor da constante é: {THREE_HOURS_IN_SECONDS}")
}
