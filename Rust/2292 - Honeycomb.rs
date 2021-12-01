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
    let n = input_integers()[0];

    if n == 1 {
        println!("1");
        return;
    }

    let mut dist = 0;

    loop {
        if 3 * dist * (dist + 1) >= n - 1 {
            break;
        }

        dist += 1;
    }

    println!("{}", dist + 1);
}
