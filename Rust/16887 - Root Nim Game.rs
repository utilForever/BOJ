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
        if *stone >= 1 && *stone <= 3 {
            ret ^= 0;
        } else if *stone >= 4 && *stone <= 15 {
            ret ^= 1;
        } else if *stone >= 16 && *stone <= 81 {
            ret ^= 2;
        } else if *stone >= 82 && *stone <= 6723 {
            ret ^= 0;
        } else if *stone >= 6724 && *stone <= 50625 {
            ret ^= 3;
        } else if *stone >= 50626 && *stone <= 2562991875 {
            ret ^= 1;
        } else {
            ret ^= 2;
        }
    }

    if ret == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}
