use std::io;
mod calculadora;
mod caraoucoroa;
fn main() {
    let mut alternativa = String::new();

    println!("1 - Cara ou coroa \n\n2 - Calculadora");
    io::stdin()
        .read_line(&mut alternativa)
        .expect("Erro ao inserir mensagem");

    alternativa = alternativa.trim().to_string();

    if alternativa.to_owned() == "1" {
        let mut resposta = String::new();
        println!("Digite qual vc quer: cara ou coroa?");
        io::stdin()
            .read_line(&mut resposta)
            .expect("Erro ao inserir mensagem");

        caraoucoroa::caraoucoroa(resposta);
    } else {
        let mut number1 = String::new();
        let mut number2 = String::new();
        println!("Calculadora");

        println!("Digite o primeiro numero:");
        io::stdin()
            .read_line(&mut number1)
            .expect("Erro ao inserir mensagem");
        println!("Digite o segundo numero:");
        io::stdin()
            .read_line(&mut number2)
            .expect("Erro ao inserir mensagem");

        let numero1: i32 = number1.trim().parse().unwrap();
        let numero2: i32 = number2.trim().parse().unwrap();

        println!(
            "A soma dos numeros é: {}",
            calculadora::soma(numero1, numero2)
        );
        println!(
            "A subtracao dos numeros é: {}",
            calculadora::subtracao(numero1, numero2)
        );
        println!(
            "A multiplicacao dos numeros é: {}",
            calculadora::multiplicacao(numero1, numero2)
        );
        println!(
            "A divisao dos numeros é: {}",
            calculadora::divisao(numero1, numero2)
        );
    }
}
