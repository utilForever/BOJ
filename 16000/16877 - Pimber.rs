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
    let stones = input_integers();

    let mut fibonacci = vec![1, 1];
    while fibonacci.last().unwrap() <= &3_000_000 {
        fibonacci.push(fibonacci[fibonacci.len() - 1] + fibonacci[fibonacci.len() - 2]);
    }

    let mut grundy = vec![0; 3_000_001];
    let mut check = vec![false; 16];

    for i in 0..=3_000_000 as usize {
        check.fill(false);

        for fibo in fibonacci.iter() {
            if *fibo <= i as i32 {
                check[grundy[i - *fibo as usize]] = true;
            } else {
                for j in 0..16 {
                    if !check[j] {
                        grundy[i] = j;
                        break;
                    }
                }
            }
        }
    }

    let mut ret = 0;
    for i in 0..n {
        ret ^= grundy[stones[i] as usize];
    }

    if ret == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}
