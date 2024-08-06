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
    let mut c = input_integers()[0] as usize;
    let mut n = 1;

    while c != 1 {
        c = if c % 2 == 0 { c / 2 } else { 3 * c + 1 };
        n += 1;
    }

    println!("{}", n);
}
