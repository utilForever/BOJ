use std::io;

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

fn main() {
    let nums = input_integers();
    let (a, b, c) = (nums[0], nums[1], nums[2]);

    let mut cur_repeat = 0;
    let mut left = (c - b) / a - 0.001;
    let mut right = (c + b) / a + 0.001;
    let mut x: f64;

    loop {
        x = (left + right) / 2.0;

        let res = a * x + b * x.sin();

        if cur_repeat > 10000000 || (res - c).abs() < 1e-15 {
            break;
        }

        if res < c {
            left = x;
        } else {
            right = x;
        }

        cur_repeat += 1;
    }

    let mut ans = (x * 10000000.0) as i64;
    ans = ans / 10 + if ans % 10 > 4 { 1 } else { 0 };

    println!("{}.{:06}", ans / 1000000, ans % 1000000);
}
