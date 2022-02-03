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
    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let n = input_integers()[0] as usize;

        if n % 2 == 0 {
            println!("cubelover");
        } else {
            println!("koosaga");
        }
    }
}
