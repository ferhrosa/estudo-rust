fn main() {
    condicao();
    repeticao();
}

fn condicao() {
    let idade: i8 = 17;
    let de_maior = idade >= 18;

    if de_maior {
        println!("Pode entrar na balada");
    }
    else {
        println!("Não pode entrar na balada!")
    }

    let condicao = if de_maior { "maior" } else { "menor" };
    println!("É {} de idade.", condicao);

    let linguagem = "C#";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido",
    };

    println!("O propósito de {} é {}", linguagem, proposito);
}

fn repeticao() {
    let multiplicador: i8 = 7;

    let mut contador: i8 = 0;
    while contador < 10 {
        contador += 1;
        if contador == 5 { continue; }
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
        if contador == 10 { break; }
    }

    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}