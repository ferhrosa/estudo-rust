fn main() {
    // Forçar error irrecuperável:
    //panic!("Erro!");

    // Erro irrecuperável no acesso ao vetor:
    //let v = vec![1, 2, 3];
    //v[4];

    tratar_resultado(resultado(true));
    tratar_resultado(resultado(false));
}

fn tratar_resultado(resultado: Result<String, u8>){
    match resultado {
        Ok(s) => println!("String de sucesso: {}", s),
        Err(n) => println!("Código de erro: {}", n),
    }
}

fn resultado(erro: bool) -> Result<String, u8> {
    if erro { Err(42) } else { Ok(String::from("Tudo deu certo")) }
}