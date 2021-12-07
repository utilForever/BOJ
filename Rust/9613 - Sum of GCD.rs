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

fn gcd(first: i32, second: i32) -> i32 {
    let mut max = first;
    let mut min = second;

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let t = input_integers()[0];

    for _ in 0..t {
        let nums = input_integers();

        let n = nums[0] as usize;

        let mut sum: i64 = 0;

        for i in 1..=n - 1 {
            for j in i + 1..=n {
                sum += gcd(nums[i], nums[j]) as i64;
            }
        }

        println!("{}", sum);
    }
}
