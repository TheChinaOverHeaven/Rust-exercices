/* Using a hash map and vectors, create a text interface to allow a user to add employee 
names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
Then let the user retrieve a list of all people in a department or all people in the company by department, 
sorted alphabetically. */

use std::collections::HashMap;
use dialoguer::{Input, Select};

fn main() {
    let mut employeers_departments: HashMap<&str, Vec<String>> = HashMap::new();
    let departments: [&str; 5] = [
        "Engineering",
        "Sales",
        "Computer Science",
        "Marketing",
        "Research and Development"
        ];
    
    loop {
        let name: String = Input::new()
            .with_prompt("Employee")
            .interact()
            .unwrap();
        let name_final: &str = name.trim();  // Only to trim.

        let department_choice: usize = Select::new()
            .with_prompt("Department")
            .items(&departments)
            .interact()
            .unwrap();
        
        let department: &str = departments[department_choice];

        employeers_departments
            .entry(department)
            .or_insert(Vec::new())
            .push(name_final.to_string());
        
        let stop: String = Input::new().with_prompt("\nWanna continue? [N to exit]")
            .interact()
            .unwrap();
        if stop.trim().to_lowercase().starts_with('n') {
            break;
        }
    }
    println!();
    for (k, v) in employeers_departments.iter_mut() {
        v.sort();

        let capitalized_names: Vec<String> = v.iter()
        .map(|name| first_letter_uppercase(name))
        .collect();

        let names_parse = capitalized_names.join(", ");
        println!("{}: {}", k, names_parse);
    }
}

fn first_letter_uppercase(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    
    let mut initial: String = word.trim().chars().next().unwrap().to_uppercase().to_string();
    let rest: &str = &word[1..].to_lowercase();
    initial += rest;
    initial
}