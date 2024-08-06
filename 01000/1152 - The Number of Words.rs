use std::io;

fn input_strings() -> Vec<String> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<String> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    println!("{}", input_strings().len());
}
