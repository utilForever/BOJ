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

fn process_bfs(visited: &mut Vec<bool>, n: usize, k: usize) -> (i64, i64) {
    let mut time_ans = -1;
    let mut count_ans = 0;

    let mut queue = VecDeque::new();
    queue.push_back((n, 0));
    visited[n] = true;

    while !queue.is_empty() {
        let (cur_n, time) = queue.pop_front().unwrap();
        visited[cur_n] = true;

        if cur_n == k {
            if time_ans == -1 {
                time_ans = time;
                count_ans += 1;    
            } else if time_ans == time {
                count_ans += 1;
            }

            visited[k] = false;
            continue;
        }

        if cur_n as i64 - 1 >= 0 && !visited[cur_n - 1] {
            queue.push_back((cur_n - 1, time + 1));
        }
        if cur_n + 1 <= 100_000 && !visited[cur_n + 1] {
            queue.push_back((cur_n + 1, time + 1));
        }
        if cur_n * 2 <= 100_000 && !visited[cur_n * 2] {
            queue.push_back((cur_n * 2, time + 1));
        }
    }

    (time_ans, count_ans)
}

fn main() {
    let nums = input_integers();
    let (n, k) = (nums[0] as usize, nums[1] as usize);

    let mut visited = vec![false; 100_001];

    let (time_ans, count_ans) = process_bfs(&mut visited, n, k);
    println!("{}", time_ans);
    println!("{}", count_ans);
}
