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
    let mut n = nums[0];
    let mut k = nums[1];

    k = if n - k > k { k } else { n - k };

    if n == 0 || k == 0 {
        println!("1");
        return;
    }

    let mut numerator = 1;
    let mut denominator = 1;

    for i in 1..=k {
        numerator *= n;
        denominator *= i;

        n -= 1;
    }

    println!("{}", numerator / denominator);
}
