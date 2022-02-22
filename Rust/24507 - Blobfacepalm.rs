use std::{io, process};

fn input_integers() -> Vec<usize> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<usize> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

// Reference: https://arxiv.org/pdf/1507.00315.pdf
fn main() {
    let n = input_integers()[0];

    if n == 4 {
        println!("Yes");
        println!("1 2 1 3 2 0 0 3");
        return;
    }

    if n == 5 {
        println!("Yes");
        println!("2 4 1 2 1 3 4 0 0 3");
        return;
    }

    match n % 4 {
        0 => {
            let mut arr = vec![0; (n + 1) * 2];
            let k = n as i64 / 4;

            for r in 0..=(2 * k - 1) {
                let val = (8 * k - r) - (4 * k + r);
                arr[(4 * k + r) as usize] = val;
                arr[(8 * k - r) as usize] = val;
            }

            {
                let val = 6 * k - (2 * k + 1);
                arr[(2 * k + 1) as usize] = val;
                arr[(6 * k) as usize] = val;
            }

            {
                let val = (4 * k - 1) - (2 * k);
                arr[(2 * k) as usize] = val;
                arr[(4 * k - 1) as usize] = val;
            }

            for r in 1..=(k - 1) {
                let val = (4 * k - 1 - r) - r;
                arr[r as usize] = val;
                arr[(4 * k - 1 - r) as usize] = val;
            }

            arr[k as usize] = 1;
            arr[(k + 1) as usize] = 1;

            for r in 0..=(k - 3) {
                let val = (3 * k - 1 - r) - (k + 2 + r);
                arr[(k + 2 + r) as usize] = val;
                arr[(3 * k - 1 - r) as usize] = val;
            }

            println!("Yes");
            for idx in 1..=(2 * n) {
                print!("{} ", arr[idx] - 1);
            }
            println!();
        }
        1 => {
            let mut arr = vec![0; (n + 1) * 2];
            let k = n as i64 / 4;

            for r in 0..=(2 * k - 1) {
                let val = (8 * k + 2 - r) - (4 * k + 2 + r);
                arr[(4 * k + 2 + r) as usize] = val;
                arr[(8 * k + 2 - r) as usize] = val;
            }

            {
                let val = (6 * k + 2) - (2 * k + 1);
                arr[(2 * k + 1) as usize] = val;
                arr[(6 * k + 2) as usize] = val;
            }

            {
                let val = (4 * k + 1) - (2 * k + 2);
                arr[(2 * k + 2) as usize] = val;
                arr[(4 * k + 1) as usize] = val;
            }

            for r in 1..=k {
                let val = (4 * k + 1 - r) - r;
                arr[r as usize] = val;
                arr[(4 * k + 1 - r) as usize] = val;
            }

            arr[(k + 1) as usize] = 1;
            arr[(k + 2) as usize] = 1;

            for r in 1..=(k - 2) {
                let val = (3 * k + 1 - r) - (k + 2 + r);
                arr[(k + 2 + r) as usize] = val;
                arr[(3 * k + 1 - r) as usize] = val;
            }

            println!("Yes");
            for idx in 1..=(2 * n) {
                print!("{} ", arr[idx] - 1);
            }
            println!();
        }
        2 => println!("No"),
        3 => println!("No"),
        _ => process::exit(1),
    }
}
