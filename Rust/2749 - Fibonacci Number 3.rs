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

    if n == 1 || n == 2 {
        println!("1");
        return;
    }

    let mut start = 1;
    let mut end = 1;
    let mut num_period = 0;

    loop {
        let tmp = (start + end) % 1_000_000;
        start = end;
        end = tmp;
        num_period += 1;

        if start == 1 && end == 1 {
            break;
        }
    }

    let mut fibonacci = vec![0_i64; num_period + 1];
    fibonacci[1] = 1;
    fibonacci[2] = 1;

    for i in 3..=num_period {
        fibonacci[i] = (fibonacci[i - 1] + fibonacci[i - 2]) % 1_000_000;
    }

    println!("{}", fibonacci[n % num_period]);
}
