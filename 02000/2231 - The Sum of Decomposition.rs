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
    let n = input_integers()[0];

    for i in 0..n {
        let mut sum = i;
        let mut num = i;

        while num >= 10 {
            sum += num % 10;
            num /= 10;
        }

        sum += num;

        if sum == n {
            println!("{}", i);
            return;
        }
    }

    println!("0");
}
