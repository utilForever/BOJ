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
    let grade = input_integers()[0];

    println!(
        "{}",
        if grade >= 90 {
            "A"
        } else if grade >= 80 {
            "B"
        } else if grade >= 70 {
            "C"
        } else if grade >= 60 {
            "D"
        } else {
            "F"
        }
    );
}
