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
    let t = input_integers()[0];

    for _ in 0..t {
        let n = input_integers()[0];

        println!(
            "{}",
            if n % 3 == 2 || n % 9 == 0 {
                "TAK"
            } else {
                "NIE"
            }
        );
    }
}
