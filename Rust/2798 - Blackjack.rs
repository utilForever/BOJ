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
    let _ = nums[0];
    let m = nums[1];

    let cards = input_integers();

    let mut max_value = 0;

    for i in 0..cards.len() - 2 {
        for j in i + 1..cards.len() - 1 {
            for k in j + 1..cards.len() {
                let sum = cards[i] + cards[j] + cards[k];

                if sum <= m && sum > max_value {
                    max_value = sum;
                }
            }
        }
    }

    println!("{}", max_value);
}
