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

fn calc_min_multiplication(
    matrix: &mut Vec<Vec<i32>>,
    min_multiplication: &mut Vec<Vec<i32>>,
    x: usize,
    y: usize,
) -> i32 {
    if min_multiplication[x][y] != 0 {
        return min_multiplication[x][y];
    }

    if x == y {
        return 0;
    }

    if x + 1 == y {
        return matrix[x][0] * matrix[x][1] * matrix[y][1];
    }

    min_multiplication[x][y] = -1;

    for i in x..=y - 1 {
        let t1 = calc_min_multiplication(matrix, min_multiplication, x, i);
        let t2 = calc_min_multiplication(matrix, min_multiplication, i + 1, y);

        if min_multiplication[x][y] == -1
            || min_multiplication[x][y] > t1 + t2 + matrix[x][0] * matrix[i][1] * matrix[y][1]
        {
            min_multiplication[x][y] = t1 + t2 + matrix[x][0] * matrix[i][1] * matrix[y][1];
        }
    }

    min_multiplication[x][y]
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut matrix = vec![vec![0; 2]; 1000];
    let mut min_multiplication = vec![vec![0; 1000]; 1000];

    for i in 0..n {
        let nums = input_integers();
        matrix[i][0] = nums[0];
        matrix[i][1] = nums[1];
    }

    println!(
        "{}",
        calc_min_multiplication(&mut matrix, &mut min_multiplication, 0, n - 1)
    );
}
