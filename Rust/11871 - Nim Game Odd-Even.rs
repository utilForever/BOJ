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
    let mut stones = input_integers();

    let mut ret = 0;
    for stone in stones.iter_mut() {
        if *stone % 2 == 0 {
            *stone = (*stone / 2) - 1;
        } else {
            *stone = (*stone + 1) / 2;
        }

        ret ^= *stone;
    }

    if ret == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}
