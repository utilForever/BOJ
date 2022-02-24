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
    let nums = input_integers();
    let (n, k) = (nums[0] as usize, nums[1] as usize);

    let tastes = input_integers();
    let mut prefix_sum_tastes = vec![0; n + k];
    prefix_sum_tastes[0] = tastes[0];
    for i in 1..(n + k) {
        prefix_sum_tastes[i] = prefix_sum_tastes[i - 1] + tastes[i % n];
    }

    let mut max_taste = 0;
    for i in k..(n + k) {
        max_taste =
            max_taste.max(prefix_sum_tastes[i] - prefix_sum_tastes[(i as i64 - k as i64) as usize]);
    }

    print!("{}", max_taste);
}
