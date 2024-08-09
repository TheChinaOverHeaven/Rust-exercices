use std::io;

pub fn input(var: &mut String) {
    io::stdin()
        .read_line(var)
        .expect("Error: can't read!");
}

pub fn conv(str: String, choice: Option<char>) -> String {
    let choice = choice.unwrap_or('f');
    let result = match choice {
        'f' => {
        let conv: f64 = str
            .trim()
            .parse()
            .expect("Error! Can't convert!");
        format!("{conv}")
        }
        'i' => {
        let conv: i64 = str
            .trim()
            .parse()
            .expect("Error! Can't convert!");
        format!("{conv}")
        }
        'u' => {
            let conv: u64 = str
                .trim()
                .parse()
                .expect("Error! Can't convert!");
            format!("{conv}")
        } 
        _ => {
            format!("Error! {choice} is not a option! Try 'f', 'i' or 'u'.")
        }
    };
    result
}