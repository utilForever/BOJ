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
    let mut ret = vec![false; n + 1];

    for i in 1..=n {
        if i as i32 - 1 >= 0 && !ret[i - 1] {
            ret[i] = true;
        }

        if i as i32 - 3 >= 0 && !ret[i - 3] {
            ret[i] = true;
        }
    }

    println!("{}", if ret[n] { "SK" } else { "CY" });
}
