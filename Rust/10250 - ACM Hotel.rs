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

    for _ in 0..t {
        let nums = input_integers();
        let h = nums[0];
        let _ = nums[1];
        let n = nums[2];

        let y = if n % h == 0 { h } else { n % h };
        let x = (n as f32 / h as f32).ceil();

        println!("{}", y * 100 + x as i32);
    }
}
