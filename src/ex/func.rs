use std::io;

pub fn input(var: &mut String) {
    io::stdin()
        .read_line(var)
        .expect("Error: can't read!");
}