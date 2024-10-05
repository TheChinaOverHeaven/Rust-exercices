/* Write a program that allows the user to store words or phrases, and each one should be stored in a 
vector. Make sure that these inputs are "permanent" and that the user can access them whenever they want. */

use std::fs;
use std::io::{Write, BufRead, BufReader};
use dialoguer::{Select, Input, Confirm};

fn main() {
    let mut index: usize = 0;
    let mut everything: Vec<String> = vec![];
    
    // This causes the program to load all the words or phrases into the vector every time it starts.
    if let Ok(file) = fs::File::open("words_or_phrases.txt") {
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            if let Ok(word) = line {
                everything.push(word);
            }
        }
    }

    match everything.get(index) {
        Some(word) => println!("{word}"),
        None => println!("List empty yet")
    }
    loop {
        let input: usize = Select::new()
            .item("↖ NEW ↘")
            .item("← BACKWARD ←")
            .item("→ FORWARD →")
            .item("● CHOICE MANUALLY ●")
            .item("↻ EDIT ↺")
            .item("∇ DELETE ∇")
            .item("⌞ LIST ⌝")
            .item("∅ EXIT ∅")
            .interact()
            .unwrap();

        match input {
            0 => {  // NEW
                let new: String = Input::new()
                    .with_prompt("Type the word or phrase")
                    .interact()
                    .unwrap();

                    let mut file: fs::File = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open("words_or_phrases.txt")
                        .expect("Failed to open file");

                writeln!(file, "{}", new).expect("Failed to write to file");

                everything.push(new);

                println!("{}", everything.get(index).unwrap_or(&"Index not found".to_string()));
            }

            1 => {  // BACKWARD
                if !everything.is_empty() {
                    if index > 0 {
                        index -= 1;
                        println!("{}", everything.get(index).unwrap_or(&"Index not found".to_string()));
                    } else {
                        println!("Index not found, already at the first index");
                        index = 0;
                        continue;
                    }
                } else {
                    println!("List is empty");
                }
            }

            2 => {  // FORWARD
                if !everything.is_empty() {
                    if index < everything.len() - 1 {
                        index += 1;
                        println!("{}", everything.get(index).unwrap_or(&"Index not found".to_string()));
                    } else {
                        println!("Index not found, already at the last index");
                        index = everything.len() - 1;
                        continue;
                    }
                } else {
                    println!("List is empty");
                }
            }

            3 => {  // CHOICE MANUALLY
                if !everything.is_empty() {
                    index = Input::new()
                        .with_prompt("Type the index to search")
                        .interact()
                        .unwrap();

                    if index >= everything.len() {
                        index = everything.len() - 1;
                    }
                    
                    println!("{}", everything.get(index).unwrap_or(&"Index not found".to_string()));
                } else {
                    println!("List is empty");
                }
            }

            4 => {  // EDIT
                if !everything.is_empty() {
                    let new_value: String = Input::new()
                        .with_prompt("Type the new value")
                        .interact()
                        .unwrap();
                    
                    everything[index] = new_value.clone();
                    
                    let mut file: fs::File = fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("words_or_phrases.txt")
                        .expect("Failed to open file");

                    for line in everything.iter() {
                        writeln!(file, "{}", line).expect("Failed to write to file");
                    }
                } else {
                    println!("List is empty");
                }
            }

            5 => {  // DELETE
                if !everything.is_empty() {
                    let verify: bool = Confirm::new()
                        .with_prompt("Are you sure? That's irreversible:")
                        .default(false)
                        .interact()
                        .unwrap();

                    if verify {
                        let mut file: fs::File = fs::OpenOptions::new()
                            .truncate(true)
                            .write(true)
                            .open("words_or_phrases.txt")
                            .expect("Failed to open file");

                        everything.remove(index);

                        for line in everything.iter() {
                            writeln!(file, "{}", line).expect("Failed to write to file");
                        
                        if index > everything.len() - 1 {
                            // Makes the index point to the last element of the vector 
                            // (which was previously the penultimate)
                            index = everything.len() - 1;
                        }

                        println!("{}", everything.get(index).unwrap_or(&"Index not found".to_string()));
                        }
                    }
                } else {
                    println!("List is empty");
                }
            }
            
            6 => {  // LIST
                if !everything.is_empty() {
                    for item in everything.iter() {
                        println!("{item}");
                    }
                } else {
                    println!("List is empty");
                }
            }

            7 => {  // EXIT
                println!("OK, exiting...");
                break;
            }

            _ => panic!("Error occurried")
        }
    }
}