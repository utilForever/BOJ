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
    let mut cnt = n;

    let mut idx = 2;

    while idx * idx <= n {
        if n % idx == 0 {
            cnt -= cnt / idx;

            while n % idx == 0 {
                n /= idx;
            }
        }

        idx += 1;
    }

    if n > 1 {
        cnt -= cnt / n;
    }

    println!("{}", cnt);
}
