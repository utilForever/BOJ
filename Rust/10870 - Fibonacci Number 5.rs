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

    if n == 0 {
        println!("0");
        return;
    } else if n == 1 || n == 2 {
        println!("1");
        return;
    }

    let mut fibonacci = vec![0; n + 1];
    fibonacci[1] = 1;
    fibonacci[2] = 1;

    for i in 3..=n {
        fibonacci[i] = fibonacci[i - 1] + fibonacci[i - 2];
    }

    println!("{}", fibonacci[n]);
}
