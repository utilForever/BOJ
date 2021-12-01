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
    let x = input_integers()[0];
    let y = input_integers()[0];

    if x > 0 && y > 0 {
        println!("1");
    } else if x < 0 && y > 0 {
        println!("2");
    } else if x < 0 && y < 0 {
        println!("3");
    } else if x > 0 && y < 0 {
        println!("4");
    }
}
