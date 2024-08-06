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
    let mut n = input_integers()[0] as usize;

    if n == 1 {
        return;
    }

    let mut i = 2;
    loop {
        if n % i == 0 {
            println!("{}", i);
            n /= i;
        } else {
            i += 1;
        }

        if i > n {
            break;
        }
    }
}
