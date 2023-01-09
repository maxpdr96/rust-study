use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let mut resposta = String::new();
    println!("Digite qual vc quer: cara ou coroa?");
    io::stdin()
        .read_line(&mut resposta)
        .expect("Erro ao inserir mensagem");
    let mut rng = thread_rng();
    let values = rng.gen_range(0..2);

    resposta = resposta.trim().to_string();

    let tupla = vec![(0, "cara"), (1, "coroa")];

    if values == tupla[0].0 && tupla[0].1 == resposta {
        println!("Parabens, vc ganhou, deu {}", tupla[0].1);
    } else {
        println!("Descupe, vc nao ganhou, deu {}", tupla[1].1);
    }

    println!("O resultado foi {}", values);
}