use std::io::{self, Write};

pub fn input(var: &mut String) {
    io::stdout()
        .flush()
        .unwrap();
    
    io::stdin()
        .read_line(var)
        .expect("Error: can't read!");
}

/* used to export functions to other file .rs */
/* 09/20/2024:
file dont useful anymore, i will only user dialoguer from now */