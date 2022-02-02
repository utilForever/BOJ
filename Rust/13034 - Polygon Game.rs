use std::{io, collections::HashSet};

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

    let mut grundy = vec![0; n + 1];

    for i in 0..=n {
        let mut s = HashSet::new();

        for j in 0..=(i as i32 - 2) {
            s.insert(grundy[j as usize] ^ grundy[i - 2 - j as usize]);
        }

        let mut j = 0;

        loop {
            if s.get(&j) == None {
                grundy[i] = j;
                break;
            }

            j += 1;
        }
    }

    if grundy[n] > 0 {
        println!("1");
    } else {
        println!("2");
    }
}
