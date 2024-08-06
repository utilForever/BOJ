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

    let n = nums[0];
    let x = nums[1];

    let nums = input_integers();

    for i in 0..n as usize {
        let num = nums[i];

        if x > num {
            print!("{} ", num);
        }
    }

    println!("");
}
