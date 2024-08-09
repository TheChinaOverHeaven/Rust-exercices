use std::io;

pub fn input(var: &mut String) {
    io::stdin()
        .read_line(var)
        .expect("Error: can't read!");
}

/* used to export functions to other file .rs */