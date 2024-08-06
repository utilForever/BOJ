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

// Reference: Problem Solving-Strategies, Chapter 1, E9
fn main() {
    let n = input_integers()[0] as usize;
    let banks = input_integers();

    let prefix_sum: i64 = banks.iter().sum();
    let mut count = 0_i64;

    for i in 0..n {
        let mut bank = banks[i];

        if bank < 0 {
            count += (-bank - 1) / prefix_sum + 1;
        }

        let mut j = (i + 1) % n;

        while i != j {
            bank += banks[j];

            if bank < 0 {
                count += (-bank - 1) / prefix_sum + 1;
            }

            j = (j + 1) % n;
        }
    }

    println!("{}", count);
}
