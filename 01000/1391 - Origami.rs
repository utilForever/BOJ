use std::{cmp, io};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
}

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
    let t = input_integers()[0];

    for _ in 0..t {
        let n = input_integers()[0];
        let nums = input_integers();

        let mut num = Vec::new();

        for i in 0..nums.len() {
            num.push(nums[i]);
        }

        let mut fold: [Vec<(i32, i32)>; 2] = [vec![], vec![]];
        let mut direction = Direction::UP;
        let mut can_fold = true;

        for j in 1..n as usize {
            let min_num = cmp::min(num[j - 1], num[j]);
            let max_num = cmp::max(num[j - 1], num[j]);

            for k in 0..fold[direction as usize].len() {
                if (min_num > fold[direction as usize][k].0
                    && min_num < fold[direction as usize][k].1
                    && max_num > fold[direction as usize][k].1)
                    || (min_num <= fold[direction as usize][k].0
                        && max_num > fold[direction as usize][k].0
                        && max_num < fold[direction as usize][k].1)
                {
                    can_fold = false;
                    break;
                }
            }

            if !can_fold {
                break;
            }

            fold[direction as usize].push((min_num, max_num));
            direction = if direction == Direction::UP {
                Direction::DOWN
            } else {
                Direction::UP
            };
        }

        println!("{}", if can_fold { "YES" } else { "NO" });
    }
}
