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
    let n = input_integers()[0] as usize;

    let mut ret = 0;
    let mut is_even = true;

    for _ in 0..n {
        let stone = input_integers()[0];
        ret ^= stone;

        if stone % 2 == 1 {
            is_even = false;
        }
    }

    if (ret == 0 && is_even) || ret == 1 {
        println!("Bob");
    } else {
        println!("Alice");
    }
}
