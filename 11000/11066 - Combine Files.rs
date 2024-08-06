use std::io;

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

fn calculate_minimum_cost(
    minimum_cost: &mut Vec<Vec<i64>>,
    accumulated_file_size: &Vec<i64>,
    i: usize,
    j: usize,
) -> i64 {
    if i == j {
        return 0;
    }

    if minimum_cost[i][j] != -1 {
        return minimum_cost[i][j];
    }

    for k in i..=(j - 1) {
        let cost = calculate_minimum_cost(minimum_cost, accumulated_file_size, i, k)
            + calculate_minimum_cost(minimum_cost, accumulated_file_size, k + 1, j)
            + accumulated_file_size[j]
            - accumulated_file_size[i - 1];

        if minimum_cost[i][j] == -1 || minimum_cost[i][j] > cost {
            minimum_cost[i][j] = cost;
        }
    }

    minimum_cost[i][j]
}

fn main() {
    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let k = input_integers()[0] as usize;

        let file_size = input_integers();
        let mut accumulated_file_size = vec![0; k + 1];
        let mut minimum_cost = vec![vec![-1; k + 1]; k + 1];

        for i in 1..=k {
            accumulated_file_size[i] = accumulated_file_size[i - 1] + file_size[i - 1];
        }

        println!(
            "{}",
            calculate_minimum_cost(&mut minimum_cost, &accumulated_file_size, 1, k)
        );
    }
}
