/* Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay 
is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead 
(apple becomes apple-hay). Keep in mind the details about UTF-8 encoding! */

use dialoguer::Input;

fn main() {
    let mut word: String = Input::new()
        .with_prompt("Type a word to convert to pig latin")
        .interact()
        .unwrap();
    
    let vogals: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    if vogals.contains(&word.to_lowercase().chars().next().unwrap()) {
        word += "hay";
        println!("Result: {word}");
    } else {
        let mut vec_result: Vec<char> = word.chars().collect();
        let first_element: Vec<char> = vec_result.clone();
        
        vec_result.remove(0);
        vec_result.push(first_element[0]);
        
        // The 'first_element' variable is cleared before the scope ends as it is useless now.
        std::mem::drop(first_element);

        let mut result: String = vec_result.iter().collect();
        result += "ay";
        println!("Result: {result}");
    }
}