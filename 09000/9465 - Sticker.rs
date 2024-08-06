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

fn main() {
    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let n = input_integers()[0] as usize;

        let mut scores = vec![vec![0; 2]; n + 1];
        let mut max_scores = vec![vec![0; 3]; n + 1];

        let nums = input_integers();
        for i in 1..=n {
            scores[i][0] = nums[i - 1];
        }

        let nums = input_integers();
        for i in 1..=n {
            scores[i][1] = nums[i - 1];
        }

        for i in 1..=n {
            max_scores[i][0] = *vec![
                max_scores[i - 1][0],
                max_scores[i - 1][1],
                max_scores[i - 1][2],
            ]
            .iter()
            .max()
            .unwrap();
            max_scores[i][1] = *vec![max_scores[i - 1][0], max_scores[i - 1][2]]
                .iter()
                .max()
                .unwrap()
                + scores[i][0];
            max_scores[i][2] = *vec![max_scores[i - 1][0], max_scores[i - 1][1]]
                .iter()
                .max()
                .unwrap()
                + scores[i][1];
        }

        println!(
            "{}",
            vec![max_scores[n][0], max_scores[n][1], max_scores[n][2]]
                .iter()
                .max()
                .unwrap()
        );
    }
}
