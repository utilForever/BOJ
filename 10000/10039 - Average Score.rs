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
    let mut sum = 0;

    for _ in 0..5 {
        let mut score = input_integers()[0];

        if score < 40 {
            score = 40;
        }

        sum += score;
    }

    println!("{}", sum / 5);
}
