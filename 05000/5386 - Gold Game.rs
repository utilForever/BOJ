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
    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let nums = input_integers();
        let (mut s, k) = (nums[0] as usize, nums[1] as usize);

        if k % 2 == 0 {
            s %= k + 1;

            if s == k {
                println!("{}", k);
            } else if s % 2 == 1 {
                println!("{}", 1);
            } else {
                println!("{}", 0);
            }
        } else {
            if s % 2 == 1 {
                println!("{}", 1);
            } else {
                println!("{}", 0);
            }
        }
    }
}
