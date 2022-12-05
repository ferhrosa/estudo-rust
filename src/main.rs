fn main() {
    println!("Olá mundo!");
    vetor();
    matriz();
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

fn matriz(){
    let matriz: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4],
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}