use std::io;

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

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
    let n = input_integers()[0];
    let nums = input_integers();

    let mut cnt_primes = 0;

    for i in 0..n as usize {
        if is_prime(nums[i]) {
            cnt_primes += 1;
        }
    }

    println!("{}", cnt_primes);
}
