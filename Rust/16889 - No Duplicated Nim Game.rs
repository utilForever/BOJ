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
        if *stone >= 1 && *stone <= 2 {
            ret ^= 1;
        } else if *stone >= 3 && *stone <= 5 {
            ret ^= 2;
        } else if *stone >= 6 && *stone <= 9 {
            ret ^= 3;
        } else if *stone >= 10 && *stone <= 14 {
            ret ^= 4;
        } else if *stone >= 15 && *stone <= 20 {
            ret ^= 5;
        } else if *stone >= 21 && *stone <= 27 {
            ret ^= 6;
        } else if *stone >= 28 && *stone <= 35 {
            ret ^= 7;
        } else if *stone >= 36 && *stone <= 44 {
            ret ^= 8;
        } else if *stone >= 45 && *stone <= 54 {
            ret ^= 9;
        } else {
            ret ^= 10;
        }
    }

    if ret == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}
