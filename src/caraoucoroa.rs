use rand::{thread_rng, Rng};

pub fn caraoucoroa(mut valor: String) -> () {
    let mut rng = thread_rng();
    let values = rng.gen_range(0..2);

    valor = valor.trim().to_string();

    let tupla = vec![(0, "cara"), (1, "coroa")];

    if values == tupla[0].0 && tupla[0].1 == valor {
        println!("Parabens, vc ganhou, deu {}", tupla[0].1);
    } else {
        println!("Descupe, vc nao ganhou, deu {}", tupla[1].1);
    }
}
