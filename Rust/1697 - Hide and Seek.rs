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

fn process_bfs(points: &mut Vec<i64>, n: usize, k: usize) {
    let mut queue = VecDeque::new();
    queue.push_back(n);

    while !queue.is_empty() {
        let cur_n = queue.pop_front().unwrap();

        if cur_n == k {
            break;
        }

        if cur_n as i64 - 1 >= 0 && points[cur_n - 1] == 0 {
            queue.push_back(cur_n - 1);
            points[cur_n - 1] = points[cur_n] + 1;
        }
        if cur_n + 1 <= 100_000 && points[cur_n + 1] == 0 {
            queue.push_back(cur_n + 1);
            points[cur_n + 1] = points[cur_n] + 1;
        }
        if cur_n * 2 <= 100_000 && points[cur_n * 2] == 0 {
            queue.push_back(cur_n * 2);
            points[cur_n * 2] = points[cur_n] + 1;
        }
    }
}

fn main() {
    let nums = input_integers();
    let (n, k) = (nums[0] as usize, nums[1] as usize);

    let mut points = vec![0; 100_001];

    process_bfs(&mut points, n, k);

    println!("{}", points[k]);
}
