use std::io;

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

const BLOCKS: [[[usize; 2]; 4]; 19] = [
    [[0, 0], [0, 1], [1, 0], [1, 1]],
    [[0, 0], [0, 1], [0, 2], [0, 3]],
    [[0, 0], [1, 0], [2, 0], [3, 0]],
    [[0, 0], [0, 1], [0, 2], [1, 0]],
    [[0, 2], [1, 0], [1, 1], [1, 2]],
    [[0, 0], [1, 0], [1, 1], [1, 2]],
    [[0, 0], [0, 1], [0, 2], [1, 2]],
    [[0, 0], [1, 0], [2, 0], [2, 1]],
    [[0, 0], [0, 1], [1, 1], [2, 1]],
    [[0, 0], [0, 1], [1, 0], [2, 0]],
    [[0, 1], [1, 1], [2, 0], [2, 1]],
    [[0, 0], [1, 0], [1, 1], [2, 1]],
    [[0, 1], [1, 0], [1, 1], [2, 0]],
    [[0, 1], [0, 2], [1, 0], [1, 1]],
    [[0, 0], [0, 1], [1, 1], [1, 2]],
    [[0, 0], [0, 1], [0, 2], [1, 1]],
    [[0, 1], [1, 0], [1, 1], [1, 2]],
    [[0, 1], [1, 0], [1, 1], [2, 1]],
    [[0, 0], [1, 0], [1, 1], [2, 0]],
];

fn get_score(nums: &Vec<Vec<i32>>, x: usize, y: usize, n: usize, m: usize, max_score: &mut i32) {
    for i in 0..19 {
        let mut res = 0;

        for j in 0..4 {
            let new_x = x + BLOCKS[i][j][0];
            let new_y = y + BLOCKS[i][j][1];

            if new_x < n && new_y < m {
                res += nums[new_x][new_y];
            }
        }

        if res > *max_score {
            *max_score = res;
        }
    }
}

fn main() {
    let nums = input_integers();
    let (n, m) = (nums[0] as usize, nums[1] as usize);

    let mut paper = vec![vec![0; m]; n];

    for i in 0..n {
        let nums = input_integers();

        for j in 0..m {
            paper[i][j] = nums[j];
        }
    }

    let mut max_score = 0;

    for i in 0..n {
        for j in 0..m {
            get_score(&paper, i, j, n, m, &mut max_score);
        }
    }

    println!("{}", max_score);
}
