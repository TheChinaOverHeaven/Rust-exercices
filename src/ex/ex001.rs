// 1. Calculadora de Média Ponderada: Crie um programa que calcula a média ponderada de uma lista de notas. 
// O usuário deve fornecer as notas e os pesos correspondentes. O programa deve calcular a média ponderada e 
// exibi-la.
mod func;
use func::input;

fn main() {
    let mut g: f64 = 0.0;
    let mut w: f64 = 0.0;
    for c in 1..4 {
        println!("Type the {c}ª grade:");
        let mut grade = String::new();
        input(&mut grade);

        println!("Type the weight:");
        let mut weight = String::new();
        input(&mut weight);

        let conv_g: f64 = grade.trim().parse().expect("Can't convert.");
        let conv_w: f64 = weight.trim().parse().expect("Can't convert.");

        g += conv_g * conv_w;
        w += conv_w;
    }
    let avarage: f64 = g / w;
    println!("The weighted average is {avarage:.2}.");
}