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
    loop {
        let nums = input_integers();
        let (a, b, c) = (nums[0], nums[1], nums[2]);

        if a == 0 && b == 0 && c == 0 {
            break;
        }

        let mut lengths = vec![a, b, c];
        lengths.sort();

        let (a, b, c) = (lengths[0], lengths[1], lengths[2]);

        if a * a + b * b == c * c {
            println!("right");
        } else {
            println!("wrong");
        }
    }
}
