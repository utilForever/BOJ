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
    let nums = input_integers();
    let (n, m) = (nums[0], nums[1]);

    let mut ret = [[1_u128; 101]; 101];

    for i in 1..=n as usize {
        for j in 1..=m as usize {
            let a = if i == j || j - 1 == 0 { 1 } else { ret[i - 1][j - 1] };
            let b = if i - 1 == j || j == 0 { 1 } else { ret[i - 1][j] };
            ret[i][j] = a + b;
        }
    }

    println!("{}", ret[n as usize][m as usize]);
}
