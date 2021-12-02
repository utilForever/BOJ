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
    let a = input_integers()[0];
    let b = input_integers()[0];
    let c = input_integers()[0];
    let d = input_integers()[0];
    let e = input_integers()[0];

    let mut total_time = 0;

    for i in a..b {
        if i < 0 {
            total_time += c;
        } else if i == 0 {
            total_time += d + e;
        } else {
            total_time += e;
        }
    }

    println!("{}", total_time);
}
