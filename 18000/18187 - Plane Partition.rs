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
    let n = input_integers()[0] as f64;

    let a = (n / 3.0).floor();
    let b = ((n + 1.0) / 3.0).floor();
    let c = ((n + 2.0) / 3.0).floor();

    let ans = (a + 1.0) * (b + 1.0) * (c + 1.0) - (a * b * c);
    println!("{}", ans);
}
