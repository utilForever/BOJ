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
    let n = input_integers()[0] as usize;
    let solutions = input_integers();

    let (mut left, mut right) = (0, n - 1);
    let (mut ans_left, mut ans_right) = (0, 0);
    let mut val = 2_000_000_001;

    while left < right {
        let left_solution = solutions[left];
        let right_solution = solutions[right];

        if (left_solution + right_solution).abs() < val {
            val = (left_solution + right_solution).abs();
            ans_left = left_solution;
            ans_right = right_solution;
        }

        if left_solution + right_solution > 0 {
            right -= 1;
        } else {
            left += 1;
        }
    }

    if ans_left > ans_right {
        println!("{} {}", ans_right, ans_left);
    } else {
        println!("{} {}", ans_left, ans_right);
    }
}
