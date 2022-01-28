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
    let s = input_integers()[0];
    let (mut sum, mut ans) = (0, 0);
    let mut idx = 1;

    loop {
        if sum + idx <= s {
            sum += idx;
            ans += 1;
        }

        if sum + idx > s {
            println!("{}", ans);
            break;
        }

        idx += 1;
    }
}
