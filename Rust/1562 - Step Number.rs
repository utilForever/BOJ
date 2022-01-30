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

fn process_dp(
    dp: &mut Vec<Vec<Vec<i64>>>,
    n: usize,
    start: usize,
    idx: usize,
    visited: usize,
) -> i64 {
    if idx == n - 1 {
        if visited == 1023 {
            return 1;
        } else {
            return 0;
        }
    }

    if dp[start][idx][visited] != -1 {
        return dp[start][idx][visited];
    }
    dp[start][idx][visited] = 0;

    for i in 0..2 {
        let next_x = start as i32 + if i == 0 { 1 } else { -1 };
        if next_x == -1 || next_x == 10 {
            continue;
        }

        dp[start][idx][visited] +=
            process_dp(dp, n, next_x as usize, idx + 1, visited | (1 << next_x));
    }

    dp[start][idx][visited] %= 1_000_000_000;
    dp[start][idx][visited]
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut dp = vec![vec![vec![-1; (1 << 10) + 1]; 101]; 11];
    let mut ans = 0;

    for i in 1..10 {
        ans += process_dp(&mut dp, n, i, 0, 1 << i);
    }

    println!("{}", ans % 1_000_000_000);
}
