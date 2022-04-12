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
    let mut a = 1;
    let mut b = 1;
    let mut c;

    for _ in 3..=n {
        c = b;
        b = (a + b) % 1_000_000_007;
        a = c;
    }

    println!("{} {}", b, n - 2);
}
