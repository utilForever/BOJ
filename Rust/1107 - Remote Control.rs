use std::{cmp, io};

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

fn move_by_number(is_broken: [bool; 10], mut num: i32) -> i32 {
    if num == 0 {
        if is_broken[0] {
            return -1;
        }

        return 1;
    }

    let mut count = 0;

    while num > 0 {
        if is_broken[(num % 10) as usize] {
            return -1;
        } else {
            count += 1;
            num /= 10;
        }
    }

    count
}

fn main() {
    let n = input_integers()[0];
    let m = input_integers()[0];

    let mut is_broken = [false; 10];

    if m > 0 {
        let nums = input_integers();

        for num in nums.iter() {
            is_broken[*num as usize] = true;
        }
    }

    if n == 100 {
        println!("0");
        return;
    }

    let mut ans = (n - 100).abs();

    for i in 0..1000000 {
        let mut count = move_by_number(is_broken, i);

        if count > 0 {
            count += (n - i).abs();
            ans = cmp::min(ans, count);
        }
    }

    println!("{}", ans);
}
