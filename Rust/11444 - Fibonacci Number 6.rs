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

fn multiply_matrix(a: Vec<Vec<i64>>, b: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut ret = vec![vec![0, 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            ret[i][j] = 0;
            
            for k in 0..2 {
                ret[i][j] += a[i][k] * b[k][j];
                ret[i][j] %= 1_000_000_007;
            }
        }
    }

    ret
}

fn get_fibonacci(n: i64) -> Vec<Vec<i64>> {
    let multiplier = vec![vec![0, 1], vec![1, 1]];

    if n == 1 {
        return multiplier;
    }

    let temp_matrix: Vec<Vec<i64>>;

    if n % 2 == 1 {
        temp_matrix = get_fibonacci(n - 1);
        return multiply_matrix(multiplier, temp_matrix.clone());
    } else {
        temp_matrix = get_fibonacci(n / 2);
        return multiply_matrix(temp_matrix.clone(), temp_matrix);
    }
}

fn main() {
    let n = input_integers()[0];

    if n == 0 {
        println!("0");
        return;
    }

    let ans = get_fibonacci(n);
    println!("{}", ans[1][0]);
}
