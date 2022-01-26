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

fn promising(r: i32, c: i32, n: i32, board: &Vec<Vec<i32>>) -> bool {
    let (mut temp_r, mut temp_c) = (r, c);

    while temp_r >= 0 && temp_c >= 0 {
        if board[temp_r as usize][temp_c as usize] == 2 {
            return false;
        }

        temp_r -= 1;
        temp_c -= 1;
    }

    temp_r = r;
    temp_c = c;

    while temp_r >= 0 && temp_c < n {
        if board[temp_r as usize][temp_c as usize] == 2 {
            return false;
        }

        temp_r -= 1;
        temp_c += 1;
    }

    true
}

fn check(
    mut r: i32,
    mut c: i32,
    count: i32,
    is_black: bool,
    n: i32,
    board: &mut Vec<Vec<i32>>,
    ans_black: &mut i32,
    ans_white: &mut i32,
) {
    if c >= n {
        r += 1;
        c = if c % 2 == 0 { 1 } else { 0 };
    }

    if r >= n {
        if is_black {
            *ans_black = cmp::max(*ans_black, count);
        } else {
            *ans_white = cmp::max(*ans_white, count);
        }
    }

    if r >= n || c >= n {
        return;
    }

    if board[r as usize][c as usize] == 1 && promising(r, c, n, board) {
        board[r as usize][c as usize] = 2;
        check(
            r,
            c + 2,
            count + 1,
            is_black,
            n,
            board,
            ans_black,
            ans_white,
        );
        board[r as usize][c as usize] = 1;
    }

    check(r, c + 2, count, is_black, n, board, ans_black, ans_white);
}

fn main() {
    let n = input_integers()[0];
    let mut board = vec![vec![0; n as usize]; n as usize];
    let (mut ans_black, mut ans_white) = (0, 0);

    for i in 0..n as usize {
        let nums = input_integers();

        for j in 0..n as usize {
            board[i][j] = nums[j];
        }
    }

    check(0, 0, 0, true, n, &mut board, &mut ans_black, &mut ans_white);
    check(0, 1, 0, false, n, &mut board, &mut ans_black, &mut ans_white);

    println!("{}", ans_black + ans_white);
}
