/* A program that takes the name, age, and gender of 4 people. Then, print the average 
age of the group, the name of the oldest man, and how many women are under 20 years old */

mod func;
use func::input;

fn main() {
    let mut name: String = String::new();
    let mut age: String = String::new();
    let mut gender: String = String::new();
    let mut women_under: u8 = 0;
    let mut ages: Vec<u8> = vec![];
    let mut names: Vec<String> = vec![];

    for _ in 0..4 {
        // ask the name
        println!("What's your name?");
        name.clear();
        input(&mut name);
        
        // remove spaces from lateral and convert to string from 'name'
        name = name.trim().to_string();
        
        // verify if all the chars of 'name' is alphabetic, if not, while loop starts
        while !name.chars().all(char::is_alphabetic) {
            println!("Only letters! What's your name?");
            // 'name' is clear
            name.clear();
            
            input(&mut name);
            name = name.trim().to_string();
        }
        
        // ask the age
        println!("What's your age?");
        age.clear();
        input(&mut age);

        age = age.trim().to_string();

        while !age.chars().all(char::is_numeric) {
            println!("Only numbers! What's your age?");
            age.clear();
            
            input(&mut age);
            age = age.trim().to_string();
        }
        // converts to a f64 number and sums to 'ages' variable
        let conv_age: u8 = age.trim().parse().expect("error");
        // stores the ages in a vector
        ages.push(conv_age);

        // ask the gender
        println!("What's your gender? [M/F]");
        gender.clear();
        input(&mut gender);

        gender = gender.trim().to_string();

        while !gender.chars().all(char::is_alphabetic) {
            println!("Only letters! What's your gender?");
            gender.clear();

            input(&mut gender);
            gender = gender.trim().to_string();
        }
        
        // stores the first character of 'gender'
        let mut first: char = gender.chars().next().unwrap_or(' ').to_lowercase().next().unwrap();
        while first != 'm' && first != 'f' {
            println!("Only 'M' or 'F'! What's your gender?");
            gender.clear();

            input(&mut gender);
            gender = gender.trim().to_string();
            while !gender.chars().all(char::is_alphabetic) {
                println!("Only letters! What's your gender?");
                gender.clear();
    
                input(&mut gender);
                gender = gender.trim().to_string();
            }

            // verify if first is not 'm' or 'f' yet
            first = gender.chars().next().unwrap_or(' ').to_lowercase().next().unwrap();
        }

        // final verifications
        if first == 'f' && conv_age < 20 {
            women_under += 1;
        }
        if first == 'm' {
            // stores the name in a vector before the knowledge of the gender
            names.push(name.clone());
        }

        // line print to let more clean on run time
        println!("----------------------------------------------------------");
    }
    let sum: u8 = ages.iter().sum();
    let sum_average = sum as f64 / 4.0;
    
    let (index, _) = ages
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, &value)| (index, value))
        .unwrap_or_else(|| (0, 0));
    
    let older: String = if !names.is_empty() { names[index].clone() } else { String::from("Unknown") };

    println!("The avarage age of the group is {:.2}", sum_average);
    println!("The name of the oldest man is {}", older);
    println!("The number of women under 20 y.o is {}", women_under);
}