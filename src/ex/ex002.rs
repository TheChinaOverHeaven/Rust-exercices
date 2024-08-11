/* Create a program that converts temperatures between Celsius and Fahrenheit. 
The user must enter the temperature and the original unit (C or F), and the program 
must convert and display the temperature in the other unit. */

mod func;
use func::input;

fn main() {
    let mut temp: String = String::new();
    let mut conv: String = String::new();

    println!("Type the temperature:");
    input(&mut temp);
    println!("Type the original unit [C/F]:");
    input(&mut conv);

    let r_temp: f64 = temp.trim().parse().expect("Failed to convert to float");
    let character: char = conv.trim().parse().expect("Failed to convert to char");
    
    if character.to_lowercase().next() == Some('c') {
        let result: f64 = (r_temp * 9.0/5.0) + 32.0;
        println!("Converted to Fahrenheit is: {result:.2}.");
    } else if character.to_lowercase().next() == Some('f') {
        let result: f64 = (r_temp - 32.0) * 5.0/9.0;
        println!("Converted to Celcius is: {result:.2}.");
    } else {
        println!("Only 'c' or 'f' please.");
    }
}