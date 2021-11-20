use std::{cmp, io};

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
    let n = input_integers()[0] as usize;

    let mut cnt = vec![0; 1000001];
    cnt[1] = 0;
    cnt[2] = 1;
    cnt[3] = 1;

    for i in 4..=n {
        if i % 2 == 0 && i % 3 == 0 {
            cnt[i] = vec![cnt[i / 2], cnt[i / 3], cnt[i - 1]]
                .iter()
                .min()
                .unwrap()
                + 1;
        } else if i % 2 == 0 {
            cnt[i] = cmp::min(cnt[i / 2], cnt[i - 1]) + 1;
        } else if i % 3 == 0 {
            cnt[i] = cmp::min(cnt[i / 3], cnt[i - 1]) + 1;
        } else {
            cnt[i] = cnt[i - 1] + 1;
        }
    }

    println!("{}", cnt[n]);
}
