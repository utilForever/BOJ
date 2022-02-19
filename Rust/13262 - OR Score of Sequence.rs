use std::{cmp, io};

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

fn calculate_maximum_score(
    cost: &Vec<Vec<i32>>,
    or_score: &mut Vec<Vec<i64>>,
    assigned_sequence_idx: &mut Vec<Vec<i64>>,
    idx: usize,
    left: i32,
    right: i32,
    p_left: usize,
    p_right: usize,
) {
    if left > right {
        return;
    }

    let mid = ((left + right) / 2) as usize;

    or_score[idx][mid] = -1;
    assigned_sequence_idx[idx][mid] = -1;

    let limit = cmp::min(mid, p_right);

    for i in p_left..=limit {
        let score = or_score[idx - 1][i - 1] + cost[i][mid] as i64;

        if or_score[idx][mid] < score {
            or_score[idx][mid] = score;
            assigned_sequence_idx[idx][mid] = i as i64;
        }
    }

    calculate_maximum_score(
        cost,
        or_score,
        assigned_sequence_idx,
        idx,
        left,
        mid as i32 - 1,
        p_left,
        assigned_sequence_idx[idx][mid] as usize,
    );
    calculate_maximum_score(
        cost,
        or_score,
        assigned_sequence_idx,
        idx,
        mid as i32 + 1,
        right,
        assigned_sequence_idx[idx][mid] as usize,
        p_right,
    );
}

fn main() {
    let nums = input_integers();
    let (n, k) = (nums[0] as usize, nums[1] as usize);

    let mut sequence = input_integers();
    sequence.insert(0, 0);

    let mut cost = vec![vec![0; n + 1]; n + 1];
    let mut or_score = vec![vec![0; n + 1]; n + 1];
    let mut assigned_sequence_idx = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        cost[i][i] = sequence[i];

        for j in (i + 1)..=n {
            cost[i][j] = cost[i][j - 1] | sequence[j];
        }
    }

    for i in 1..=n {
        or_score[1][i] = cost[1][i] as i64;
        assigned_sequence_idx[1][i] = 1;
    }

    for i in 2..=k {
        calculate_maximum_score(
            &cost,
            &mut or_score,
            &mut assigned_sequence_idx,
            i,
            i as i32,
            n as i32,
            i,
            n,
        );
    }

    println!("{}", or_score[k][n]);
}
