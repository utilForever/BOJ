use std::{collections::VecDeque, io};

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

fn process_bfs(image: &Vec<Vec<char>>, checked: &mut Vec<Vec<bool>>, n: usize, c: char) -> i32 {
    let mut queue = VecDeque::new();
    let mut count = 0;

    for i in 0..n {
        for j in 0..n {
            if image[i][j] == c && !checked[i][j] {
                queue.push_back((j as i32, i as i32));
                checked[i][j] = true;
                count += 1;
            }

            while !queue.is_empty() {
                let (cur_x, cur_y) = queue.pop_front().unwrap();

                for k in 0..4 {
                    let next_x = cur_x
                        + if k == 0 {
                            -1
                        } else if k == 1 {
                            1
                        } else {
                            0
                        };
                    let next_y = cur_y
                        + if k == 2 {
                            -1
                        } else if k == 3 {
                            1
                        } else {
                            0
                        };

                    if next_x < 0 || next_x >= n as i32 || next_y < 0 || next_y >= n as i32 {
                        continue;
                    }

                    if checked[next_y as usize][next_x as usize] {
                        continue;
                    }

                    if image[next_y as usize][next_x as usize] == c {
                        queue.push_back((next_x, next_y));
                        checked[next_y as usize][next_x as usize] = true;
                    }
                }
            }
        }
    }

    count
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut image = vec![vec!['\0'; n]; n];
    let mut checked = vec![vec![false; n]; n];

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let mut chars = s.chars();

        for j in 0..n {
            image[i][j] = chars.next().unwrap();
        }
    }

    print!(
        "{} ",
        process_bfs(&image, &mut checked, n, 'R')
            + process_bfs(&image, &mut checked, n, 'G')
            + process_bfs(&image, &mut checked, n, 'B')
    );

    for i in 0..n {
        for j in 0..n {
            checked[i][j] = false;

            if image[i][j] == 'G' {
                image[i][j] = 'R';
            }
        }
    }

    println!(
        "{}",
        process_bfs(&image, &mut checked, n, 'R') + process_bfs(&image, &mut checked, n, 'B')
    );
}
