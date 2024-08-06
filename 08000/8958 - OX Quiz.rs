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
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let mut sum = 0;
        let mut score = 1;

        for c in s.chars() {
            if c == 'O' {
                sum += score;
                score += 1;
            } else {
                score = 1;
            }
        }

        println!("{}", sum);
    }
}
