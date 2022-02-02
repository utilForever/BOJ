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
    let stones = input_integers();

    let mut ans = 0;

    for i in 0..n {
        let mut ret = 0;

        for j in 0..n {
            if i == j {
                continue;
            }

            ret ^= stones[j];
        }

        for j in 0..stones[i] {
            if (ret ^ j) == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
