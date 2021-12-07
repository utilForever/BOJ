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
    let n = input_integers()[0] as usize;

    for i in (1..=n).rev() {
        for _ in (i..=(n - 1)).rev() {
            print!(" ");
        }
        for _ in 1..=(2 * i - 1) {
            print!("*");
        }

        println!();
    }
}
