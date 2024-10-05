/* Given a list of integers, use a vector and return the median 
(when sorted, the value in the middle position) and mode (the value that occurs most often) of the list. */

use std::collections::HashMap;

fn main() {
    // median
    let mut list: Vec<i8> = vec![1, 8, 9, -3, -8, 3, 8, 3, -6, 4, 1];
    list.sort();
    
    let middle = list.len() / 2 - 1;

    // mode
    let mut mode: HashMap<i8, i8> = HashMap::new();

    for num in &list {
        let quantity = mode.entry(*num).or_insert(0);
        *quantity += 1;
    }

    let mut most_used: i8 = 0;  // The number most used, we define it with 0 to start.
    
    // The quantity of times that the number appears, we define it with the first element of the hashmap.
    let mut mode_num: i8 = list[0];

    for (k, v) in &mode {
        if *v > most_used {
            most_used = *v;
            mode_num = *k;
        }
    }

    // result
    println!("The median of {:?} is {:?}", list, list[middle]);
    list.sort();
    println!("The mode of {:?} is {:?} and it appears {:?} times", list, mode_num, most_used);
}