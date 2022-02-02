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
    let _ = input_integers()[0] as usize;
    let stones = input_integers();

    let mut ret = 0;
    for stone in stones.iter() {
        ret ^= stone;
    }

    if ret == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}
