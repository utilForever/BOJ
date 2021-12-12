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

    let mut res = a * b * c;
    let mut arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    while res >= 10 {
        arr[(res % 10) as usize] += 1;
        res /= 10;
    }

    arr[res as usize] += 1;

    for i in 0..10 {
        println!("{}", arr[i]);
    }
}
