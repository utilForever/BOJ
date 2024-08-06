use std::{collections::VecDeque, io};

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

fn process_bfs(a: i64, b: i64) -> i64 {
    let mut queue = VecDeque::new();
    queue.push_back((a, 0));

    while !queue.is_empty() {
        let (cur_n, cnt) = queue.pop_front().unwrap();

        if cur_n == b {
            return cnt + 1;
        }

        if cur_n * 2 <= b {
            queue.push_back((cur_n * 2, cnt + 1));
        }
        if cur_n * 10 + 1 <= b {
            queue.push_back((cur_n * 10 + 1, cnt + 1));
        }
    }

    -1
}

fn main() {
    let nums = input_integers();
    let (a, b) = (nums[0], nums[1]);

    println!("{}", process_bfs(a, b));
}
