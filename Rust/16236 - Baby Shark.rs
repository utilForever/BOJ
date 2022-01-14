use std::{collections::BinaryHeap, io};

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

fn process_bfs(rooms: &mut Vec<Vec<i64>>, n: usize) -> i64 {
    let mut queue = BinaryHeap::new();
    let mut check = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            if rooms[i][j] == 9 {
                queue.push((0, -(i as i64), -(j as i64)));
                rooms[i][j] = 0;
                break;
            }
        }
    }

    let mut shark_size = 02;
    let mut num_eat = 0;
    let mut total_second = 0;

    while !queue.is_empty() {
        let (mut dist, mut cur_y, mut cur_x) = queue.pop().unwrap();
        dist *= -1;
        cur_y *= -1;
        cur_x *= -1;

        if rooms[cur_y as usize][cur_x as usize] > 0
            && rooms[cur_y as usize][cur_x as usize] < shark_size
        {
            num_eat += 1;

            if num_eat == shark_size {
                shark_size += 1;
                num_eat = 0;
            }

            rooms[cur_y as usize][cur_x as usize] = 0;
            total_second += dist;

            dist = 0;

            for i in 0..n {
                for j in 0..n {
                    check[i][j] = false;
                }
            }

            while !queue.is_empty() {
                queue.pop();
            }
        }

        for i in 0..4 {
            let next_x = cur_x
                + if i == 1 {
                    -1
                } else if i == 2 {
                    1
                } else {
                    0
                };
            let next_y = cur_y
                + if i == 0 {
                    -1
                } else if i == 3 {
                    1
                } else {
                    0
                };

            if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= n as i64 {
                continue;
            }

            if check[next_y as usize][next_x as usize] {
                continue;
            }

            if rooms[next_y as usize][next_x as usize] > 0
                && rooms[next_y as usize][next_x as usize] > shark_size
            {
                continue;
            }

            queue.push((-(dist + 1), -next_y, -next_x));
            check[next_y as usize][next_x as usize] = true;
        }
    }

    total_second
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut rooms = vec![vec![0; n]; n];

    for i in 0..n {
        let nums = input_integers();
        for j in 0..n {
            rooms[i][j] = nums[j];
        }
    }

    println!("{}", process_bfs(&mut rooms, n));
}
