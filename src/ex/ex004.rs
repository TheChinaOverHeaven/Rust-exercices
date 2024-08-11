/* Take input from a user and check if it is a palindrome */
mod func;
use func::input;

fn main() {
    let mut palin = String::new();
    println!("Type a phrase or a word:");
    input(&mut palin);

    // remove spaces and turn it to lower case
    let clean_palin: String = palin.trim().to_lowercase();

    // revert the string
    let rev_s: String = clean_palin.chars().rev().collect();

    if rev_s == clean_palin {
        println!("It is a palindrome.");
        println!("{} = {}", clean_palin, rev_s);
    } else {
        println!("It is not a palindrome.");
        println!("{} != {}", clean_palin, rev_s);
    }
}