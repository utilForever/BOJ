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
    let mut longest = vec![0; n];

    for i in 0..n {
        longest[i] = 1;

        for j in 0..i {
            if sequence[i] > sequence[j] && longest[i] < longest[j] + 1 {
                longest[i] = longest[j] + 1;
            }
        }
    }

    let mut ans = longest.iter().max().unwrap().clone();
    println!("{}", ans);

    let mut nums = Vec::new();
    for i in (0..n).rev() {
        if longest[i] == ans {
            nums.push(sequence[i]);
            ans -= 1;
        }
    }

    while !nums.is_empty() {
        print!("{} ", nums.pop().unwrap());
    }

    println!();
}
