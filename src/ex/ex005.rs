// Take input of a number and calculate it's factorial
fn fac(n: u64) -> u64 {
    let mut f: u64 = 1;
    for c in (1..=n).rev() {
        f *= c;
    }
    f
}
mod func;
use func::input;

fn main() {
    let mut num = String::new();
    println!("type a num:");
    input(&mut num);

    let conv: u64 = num.trim().parse().expect("Can not convert");
    println!("The factorial of {conv} is {}", fac(conv));
}