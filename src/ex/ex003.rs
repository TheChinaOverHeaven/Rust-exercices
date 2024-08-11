/* Write a program that counts the number of words in a user-supplied string. 
The program must consider spaces as word delimiters and display the total number of words. */
mod func;
use func::input;

fn main() {
    let mut phrase: String = String::new();
    println!("Type a phrase:");
    input(&mut phrase);

    let conv: &str = phrase.trim();
    let split: std::str::Split<char> = conv.split(' ');
    let mut counter: u32 = 0;
    for _ in split {
        counter += 1;
    }
    println!("There's {counter} words in '{conv}'.")
}