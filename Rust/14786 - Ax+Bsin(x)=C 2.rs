use std::{f64::consts::PI, io};

fn input_integers() -> Vec<f64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<f64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn sin(mut x: f64) -> f64 {
    x %= 2.0 * PI;

    let (mut i, mut last_s, mut s, mut fact, mut num, mut sign) = (1.0, 0.0, x, 1.0, x, 1.0);

    while s != last_s {
        last_s = s;
        i += 2.0;
        fact *= i * (i - 1.0);
        num *= x * x;
        sign *= -1.0;
        s += num / fact * sign;
    }

    s
}

fn main() {
    let nums = input_integers();
    let (a, b, c) = (nums[0], nums[1], nums[2]);

    let mut left = 0.0;
    let mut right = 200000.0;
    let mut x = 0.0;

    while right - left > 1e-9 {
        x = (left + right) / 2.0;

        if a * x + b * sin(x) < c {
            left = x - 1e-10;
        } else {
            right = x + 1e-10;
        }
    }

    println!("{}", x);
}
