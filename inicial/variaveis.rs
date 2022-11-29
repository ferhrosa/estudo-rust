const PI: f32 = 3.14;
static GLOBAL: i8 = 123;

fn main() {
    println!("PI = {}", PI);
    println!("Global = {}", GLOBAL);

    let variavel: u8 = 128;
    println!("Variável = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

    let variavel: bool = true;
    println!("Variável = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

    let decimal: f32 = 2.5;
    println!("Decimal = {}", decimal);
}
