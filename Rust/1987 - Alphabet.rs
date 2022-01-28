use std::{cmp, io};

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

fn promising(
    cur_r: i32,
    cur_c: i32,
    r: i32,
    c: i32,
    board: &Vec<Vec<char>>,
    alphabet: &Vec<i32>,
) -> bool {
    if cur_r < 0 || cur_r >= r || cur_c < 0 || cur_c >= c {
        return false;
    }

    let char = board[cur_r as usize][cur_c as usize];
    if alphabet[(char as i32 - 'A' as i32) as usize] > 0 {
        return false;
    }

    true
}

fn check(
    cur_r: i32,
    cur_c: i32,
    r: i32,
    c: i32,
    count: i32,
    board: &Vec<Vec<char>>,
    alphabet: &mut Vec<i32>,
    ans: &mut i32,
) {
    *ans = cmp::max(*ans, count);

    for i in 0..4 {
        if promising(cur_r, cur_c, r, c, board, alphabet) {
            let char = board[cur_r as usize][cur_c as usize];
            let next_r = cur_r
                + if i == 1 {
                    -1
                } else if i == 2 {
                    1
                } else {
                    0
                };
            let next_c = cur_c
                + if i == 0 {
                    -1
                } else if i == 3 {
                    1
                } else {
                    0
                };

            alphabet[(char as i32 - 'A' as i32) as usize] += 1;
            check(next_r, next_c, r, c, count + 1, board, alphabet, ans);
            alphabet[(char as i32 - 'A' as i32) as usize] -= 1;
        }
    }
}

fn main() {
    let nums = input_integers();
    let (r, c) = (nums[0] as usize, nums[1] as usize);

    let mut board = vec![vec![' '; c]; r];
    let mut alphabet = vec![0; 26];
    let mut ans = 0;

    for i in 0..r {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        let mut chars = s.chars();

        for j in 0..c {
            board[i][j] = chars.next().unwrap();
        }
    }

    check(0, 0, r as i32, c as i32, 0, &board, &mut alphabet, &mut ans);
    println!("{}", ans);
}
