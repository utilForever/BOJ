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
    let mut n = input_integers()[0].abs();

    if n == 0 {
        println!("0");
    } else if n % 2 == 0 {
        println!("-1");
    } else {
        let mut cnt = 0;

        while n > 0 {
            n /= 2;
            cnt += 1;
        }

        println!("{}", cnt);
    }
}
