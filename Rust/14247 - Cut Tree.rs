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
    let n = input_integers()[0] as usize;

    let mut sum = 0;

    let hi = input_integers();
    sum += hi.iter().sum::<i64>();

    let mut ai = input_integers();
    ai.sort();

    for i in 0..n {
        sum += ai[i] * i as i64;
    }

    println!("{}", sum);
}
