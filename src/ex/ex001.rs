// 1. Calculadora de Média Ponderada: Crie um programa que calcula a média ponderada de uma lista de notas. 
// O usuário deve fornecer as notas e os pesos correspondentes. O programa deve calcular a média ponderada e 
// exibi-la.
mod func;
use func::input;

fn main() {
    // notas:
    println!("Diga as notas dos três alunos.");
    
    let mut nota1 = String::new();
    let mut nota2 = String::new();
    let mut nota3 = String::new();

    // faz a parte do input das notas
    println!("Nota 1: ");
    input(&mut nota1);
    
    println!("Nota 2: ");
    input(&mut nota2);

    println!("Nota 3: ");
    input(&mut nota3);

    // converte para números (floats)
    let conv1: f64 = nota1.trim().parse().expect("Erro ao converter!");
    let conv2: f64 = nota2.trim().parse().expect("Erro ao converter!");
    let conv3: f64 = nota3.trim().parse().expect("Erro ao converter!");

    println!("As notas foram: {conv1}, {conv2} e {conv3}");

    // pesos:
    println!("Diga o peso das notas em ordem respectiva.");

    let mut peso1 = String::new();
    let mut peso2 = String::new();
    let mut peso3 = String::new();

    // faz a parte do input do peso das notas
    println!("Peso 1: ");
    input(&mut peso1);
    
    println!("Peso 2: ");
    input(&mut peso2);

    println!("Peso 3: ");
    input(&mut peso3);

    // converte para números (floats)
    let conv_1: f64 = peso1.trim().parse().expect("Erro ao converter!");
    let conv_2: f64 = peso2.trim().parse().expect("Erro ao converter!");
    let conv_3: f64 = peso3.trim().parse().expect("Erro ao converter!");

    println!("Os pesos foram: {conv_1}, {conv_2} e {conv_3}");

    // cálculo da média ponderada e output
    let media = ((conv1*conv_1) + (conv2*conv_2) + (conv3*conv_3)) / (conv_1 + conv_2 + conv_3);
    println!("A média ponderada foi: {media:.2}.");
}