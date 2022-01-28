use std::{collections::HashSet, io};

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

fn process_dfs(
    maze: &Vec<Vec<i32>>,
    numbered_maze: &mut Vec<Vec<i32>>,
    n: i32,
    m: i32,
    x: i32,
    y: i32,
    num_empty: i32,
    visited: &mut Vec<Vec<bool>>,
) -> i32 {
    let mut ret = 1;

    numbered_maze[y as usize][x as usize] = num_empty;
    visited[y as usize][x as usize] = true;

    for i in 0..4 {
        let next_x = x + if i == 0 {
            -1
        } else if i == 1 {
            1
        } else {
            0
        };
        let next_y = y + if i == 2 {
            -1
        } else if i == 3 {
            1
        } else {
            0
        };

        if next_x < 0 || next_x >= m || next_y < 0 || next_y >= n {
            continue;
        }

        if !visited[next_y as usize][next_x as usize] && maze[next_y as usize][next_x as usize] == 0
        {
            ret += process_dfs(
                maze,
                numbered_maze,
                n,
                m,
                next_x,
                next_y,
                num_empty,
                visited,
            )
        }
    }

    ret
}

fn process_flood_fill(maze: &mut Vec<Vec<i32>>, n: i32, m: i32) {
    let mut numbered_maze = vec![vec![0; m as usize]; n as usize];
    let mut visited = vec![vec![false; m as usize]; n as usize];
    let mut vec = Vec::new();
    let mut num_empty = 1;

    vec.push(0);

    for i in 0..n {
        for j in 0..m {
            if maze[i as usize][j as usize] == 0 && !visited[i as usize][j as usize] {
                vec.push(process_dfs(
                    &maze,
                    &mut numbered_maze,
                    n,
                    m,
                    j,
                    i,
                    num_empty,
                    &mut visited,
                ));
                num_empty += 1;
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if maze[i as usize][j as usize] == 1 {
                let mut set = HashSet::new();

                for k in 0..4 {
                    let next_x = j + if k == 0 {
                        -1
                    } else if k == 1 {
                        1
                    } else {
                        0
                    };
                    let next_y = i + if k == 2 {
                        -1
                    } else if k == 3 {
                        1
                    } else {
                        0
                    };

                    if next_x < 0 || next_x >= m || next_y < 0 || next_y >= n {
                        continue;
                    }

                    set.insert(numbered_maze[next_y as usize][next_x as usize]);
                }

                for k in set.iter() {
                    maze[i as usize][j as usize] += vec[*k as usize];
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            print!("{}", maze[i as usize][j as usize] % 10);
        }
        println!();
    }
}

fn main() {
    let nums = input_integers();
    let (n, m) = (nums[0] as usize, nums[1] as usize);

    let mut maze = vec![vec![0; m]; n];

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        let mut chars = s.chars();

        for j in 0..m {
            maze[i][j] = chars.next().unwrap() as i32 - '0' as i32;
        }
    }

    process_flood_fill(&mut maze, n as i32, m as i32);
}
