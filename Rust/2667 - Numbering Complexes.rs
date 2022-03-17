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

fn process_bfs(map: &Vec<Vec<char>>, n: usize) {
    let mut queue = VecDeque::new();
    let mut checked = vec![vec![false; n]; n];
    let mut num_houses = Vec::new();
    let mut numbering_count = 0;

    for i in 0..n {
        for j in 0..n {
            let mut num_house = 0;

            if map[i][j] == '1' && !checked[i][j] {
                checked[i][j] = true;
                queue.push_back((j, i));
                numbering_count += 1;
            }

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();

                for i in 0..4 {
                    let (dx, dy) = match i {
                        0 => (0, -1),
                        1 => (1, 0),
                        2 => (0, 1),
                        3 => (-1, 0),
                        _ => (0, 0),
                    };

                    let (next_x, next_y) = (x as i32 + dx, y as i32 + dy);

                    if next_x < 0 || next_x >= n as i32 || next_y < 0 || next_y >= n as i32 {
                        continue;
                    }

                    let next_x = next_x as usize;
                    let next_y = next_y as usize;

                    if checked[next_y][next_x] {
                        continue;
                    }

                    if map[next_y][next_x] == '1' {
                        checked[next_y][next_x] = true;
                        queue.push_back((next_x, next_y));
                    }
                }

                num_house += 1;
            }

            if num_house > 0 {
                num_houses.push(num_house);
            }
        }
    }

    println!("{}", numbering_count);

    num_houses.sort();

    for num_house in num_houses.iter() {
        println!("{}", num_house);
    }
}

fn main() {
    let n = input_integers()[0] as usize;
    let mut map = vec![vec!['0'; n]; n];

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        let mut chars = s.chars();

        for j in 0..n {
            map[i][j] = chars.next().unwrap();
        }
    }

    process_bfs(&map, n);
}
