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

    if n < 100 {
        println!("{}", n);
        return;
    }

    let mut cnt = 99;

    for i in 100..=n {
        let first = i / 100;
        let second = (i / 10) % 10;
        let third = i % 10;

        if first + third == 2 * second {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
