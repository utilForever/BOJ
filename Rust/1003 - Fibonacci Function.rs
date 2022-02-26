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
    let mut fibonacci = vec![vec![0_i64; 2]; 41];

    fibonacci[0][0] = 1;
    fibonacci[0][1] = 0;
    fibonacci[1][0] = 0;
    fibonacci[1][1] = 1;

    for i in 2..=40 {
        fibonacci[i][0] = fibonacci[i - 1][0] + fibonacci[i - 2][0];
        fibonacci[i][1] = fibonacci[i - 1][1] + fibonacci[i - 2][1];
    }

    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let n = input_integers()[0] as usize;
        println!("{} {}", fibonacci[n][0], fibonacci[n][1]);
    }
}
