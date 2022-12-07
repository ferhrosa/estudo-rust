fn main() {
    println!("Olá mundo!");
    vetor();
    matriz();
    enums();
    conteudo_opcional();
}

fn vetor() {
    // let notas = [6.5; 4];
    let notas = [10f32, 8f32, 9.5, 6.0];

    println!(
        "Nota 1 = {}, Nota 2 = {}, Nota 3 {}, Nota 4 = {}",
        notas[0], notas[1], notas[2], notas[3]
    );

    for nota in notas {
        println!("A nota é = {}", nota);
    }

    for i in 0..notas.len() {
        println!("A nota {} é = {}", i + 1, notas[i]);
    }
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

fn enums() {
    let dia_da_semana = DiaDaSemana::Terca;
    println!("É final de semana? {}", eh_final_de_semana(dia_da_semana));

    cores();
}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

fn eh_final_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false,
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn cores() {
    // let cor = Color::Blue;
    // let cor = Color::RgbColor(255, 255, 255);
    let cor = Color::CmykColor {
        cyan: 20,
        magenta: 30,
        yellow: 40,
        black: 50,
    };

    println!(
        "Cor = {}",
        match cor {
            Color::Red => "vermelho",
            Color::Green => "verde",
            Color::Blue => "azul",
            Color::RgbColor(0, 0, 0) => "preto",
            Color::RgbColor(_, _, _)
            | Color::CmykColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "RGB desconhecido",
            Color::CmykColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "CMYK desconhecido",
        }
    );
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from("existente"));
    // let conteudo_arquivo = ler_arquivo(String::from("não existente"));

    match &conteudo_arquivo {
        Some(conteudo) => println!("Arquivo encontrado: {}", conteudo),
        None => println!("Arquivo não existe."),
    }

    println!("{:?}", conteudo_arquivo);

    if let Some(conteudo) = conteudo_arquivo {
        println!("Agora tenho certeza que há conteúdo: {}", conteudo);
    }
    // Também é possível fazer while let
}

fn ler_arquivo(caminho: String) -> Option<String> {
    match caminho.as_str() {
        "existente" => Some(String::from("Conteúdo do arquivo")),
        _ => None,
    }
}
