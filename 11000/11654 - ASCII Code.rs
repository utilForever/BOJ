use std::io;

fn input_chars() -> Vec<char> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<char> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let c = input_chars()[0];
    println!("{}", c as i32);
}
