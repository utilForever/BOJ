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

    let sequence = input_integers();
    let mut longest_increase = vec![0; n];
    let mut longest_decrease = vec![0; n];

    for i in 0..n {
        longest_increase[i] = 1;

        for j in 0..i {
            if sequence[i] > sequence[j] && longest_increase[i] < longest_increase[j] + 1 {
                longest_increase[i] = longest_increase[j] + 1;
            }
        }
    }

    for i in (0..n).rev() {
        longest_decrease[i] = 1;

        for j in (i..n).rev() {
            if sequence[i] > sequence[j] && longest_decrease[i] < longest_decrease[j] + 1 {
                longest_decrease[i] = longest_decrease[j] + 1;
            }
        }
    }

    let mut ans = 0;

    for i in 0..n {
        if ans < longest_increase[i] + longest_decrease[i] - 1 {
            ans = longest_increase[i] + longest_decrease[i] - 1;
        }
    }

    println!("{}", ans);
}
