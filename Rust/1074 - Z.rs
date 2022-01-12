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
    let nums = input_integers();
    let (mut n, mut r, mut c) = (nums[0], nums[1], nums[2]);

    let mut mid;
    let mut ans = 0;

    while n > 0 {
        let mut side = 0;
        mid = 2_i32.pow(n as u32) / 2;

        if r < mid && c < mid {
            side = 0;
        } else if r < mid && c >= mid {
            side = 1;
        } else if r >= mid && c < mid {
            side = 2;
        } else if r >= mid && c >= mid {
            side = 3;
        }

        c %= mid;
        r %= mid;

        ans += mid.pow(2) * side;
        n -= 1;
    }

    println!("{}", ans);
}
