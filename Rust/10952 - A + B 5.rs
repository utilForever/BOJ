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
    loop {
        let nums = input_integers();

        let a = nums[0];
        let b = nums[1];

        if a == 0 && b == 0 {
            break;
        }

        println!("{}", a + b);
    }
}
