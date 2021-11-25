use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let year = input_integers()[0];

    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
