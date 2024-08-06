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
    let nums = input_integers();
    let (mut n, m) = (nums[0] as usize, nums[1] as usize);

    let mut x = vec![0; n];
    let mut l = vec![vec![vec![0; 2]; n + 1]; n + 1];
    let mut r = vec![vec![vec![0; 2]; n + 1]; n + 1];

    for i in 0..n {
        x[i] = input_integers()[0];
    }

    let mut is_zero = false;

    for i in 0..n {
        if x[i] == 0 {
            is_zero = true;
            break;
        }
    }

    if !is_zero {
        x.insert(n, 0);
        n += 1;
    }

    x.sort();

    let mut pos_start = 0;

    for i in 0..n {
        if x[i] == 0 {
            pos_start = i;
            break;
        }
    }

    let mut max_candy = 0;

    for k in 1..n {
        for i in 0..n {
            for j in i..n {
                l[i][j][k % 2] = 100_000_000;
                r[i][j][k % 2] = 100_000_000;

                if i >= 1 {
                    l[i][j][k % 2] = std::cmp::min(
                        l[i][j][k % 2],
                        l[i - 1][j][(k + 1) % 2] + k as i64 * (x[i] - x[i - 1]),
                    );
                    r[i][j][k % 2] = std::cmp::min(
                        r[i][j][k % 2],
                        l[i - 1][j][(k + 1) % 2] + k as i64 * (x[j] - x[i - 1]),
                    );
                }

                if j < n - 1 {
                    l[i][j][k % 2] = std::cmp::min(
                        l[i][j][k % 2],
                        r[i][j + 1][(k + 1) % 2] + k as i64 * (x[j + 1] - x[i]),
                    );
                    r[i][j][k % 2] = std::cmp::min(
                        r[i][j][k % 2],
                        r[i][j + 1][(k + 1) % 2] + k as i64 * (x[j + 1] - x[j]),
                    );
                }
            }
        }

        max_candy = std::cmp::max(
            max_candy,
            k as i64 * m as i64 - l[pos_start][pos_start][k % 2],
        );
    }

    if is_zero {
        max_candy += m as i64;
    }

    println!("{}", max_candy);
}
