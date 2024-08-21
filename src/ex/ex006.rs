/* Make a 1st-degree function calculator */
mod func;
use func::input;

fn main() {
    println!("f(x) = ax + b\n");
    let mut a = String::new();
    let mut b = String::new();

    
    println!("What's 'a' value?");
    input(&mut a);
    println!("Whats's 'b' value?");
    input(&mut b);

    let conv_a:f64 = a.trim().parse().expect("Failed to convert");
    let conv_b:f64 = b.trim().parse().expect("Failed to convert");

    let x = conv_b * (-1.0) / conv_a;

    println!("The 'x' value is: {x}")
}