use std::io;

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let a = input_integers()[0];

    for i in 1..=a {
        if 30 % (i + 1) == 0 {
            println!("{}", i);
        }
    }
}
