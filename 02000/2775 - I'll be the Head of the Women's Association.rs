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
    let mut apartment = [[0; 15]; 15];

    for i in 0..15 {
        apartment[0][i] = i;
    }

    for i in 1..15 {
        for j in 0..15 {
            let mut people = 0;

            for k in 0..=j {
                people += apartment[i - 1][k];
            }

            apartment[i][j] = people;
        }
    }

    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let k = input_integers()[0] as usize;
        let n = input_integers()[0] as usize;

        println!("{}", apartment[k][n]);
    }
}
