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
    let n = input_integers()[0] as usize;

    let sequence = input_integers();
    let mut biggest = vec![0; n];

    for i in 0..n {
        biggest[i] = sequence[i];

        for j in 0..i {
            if sequence[j] < sequence[i] && biggest[j] + sequence[i] > biggest[i] {
                biggest[i] = biggest[j] + sequence[i];
            }
        }
    }

    println!("{}", biggest.iter().max().unwrap());
}
