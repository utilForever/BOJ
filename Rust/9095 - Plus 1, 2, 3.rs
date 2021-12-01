use std::io;

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
    let t = input_integers()[0];

    let mut cnt = vec![0; 11];
    cnt[1] = 1;
    cnt[2] = 2;
    cnt[3] = 4;

    for i in 4..=10 {
        cnt[i] = cnt[i - 1] + cnt[i - 2] + cnt[i - 3];
    }

    for _ in 0..t {
        let idx = input_integers()[0];

        println!("{}", cnt[idx as usize]);
    }
}
