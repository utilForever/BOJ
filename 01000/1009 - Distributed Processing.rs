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

fn main() {
    let ans = [
        [10, 10, 10, 10],
        [1, 1, 1, 1],
        [6, 2, 4, 8],
        [1, 3, 9, 7],
        [6, 4, 6, 4],
        [5, 5, 5, 5],
        [6, 6, 6, 6],
        [1, 7, 9, 3],
        [6, 8, 4, 2],
        [1, 9, 1, 9],
    ];

    let t = input_integers()[0];

    for _ in 0..t {
        let nums = input_integers();
        let a = nums[0];
        let b = nums[1];

        println!("{}", ans[a as usize % 10][b as usize % 4]);
    }
}
