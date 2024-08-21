/* Make a 2nd-degree function calculator */
mod func;
use func::input;

fn main() {
    println!("f(x) = ax² + bx + c\n");
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    
    println!("What's the value of 'a'?");
    input(&mut a);
    println!("What's the value of 'b'?");
    input(&mut b);
    println!("What's the value of 'c'?");
    input(&mut c);

    let conv_a:f64 = a.trim().parse().expect("Failed to convert");
    let conv_b:f64 = b.trim().parse().expect("Failed to convert");
    let conv_c:f64 = c.trim().parse().expect("Failed to convert");

    let delta: f64 = conv_b.powf(2.0) - 4.0 * conv_a * conv_c;
    let x1: f64 = (conv_b * (-1.0) + delta.sqrt()) / 2.0 * conv_a;
    let x2: f64 = (conv_b * (-1.0) - delta.sqrt()) / 2.0 * conv_a;

    println!("The value of 'x₁' is: {x1}\nThe value of 'x₂' is: {x2}")
}