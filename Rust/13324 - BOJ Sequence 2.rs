use std::{cmp, collections::BinaryHeap, io};
use io::Write;

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
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = input_integers()[0];

    let mut queue = BinaryHeap::new();
    let mut ans = vec![0; 1000000];

    let nums = input_integers();

    for i in 0..n {
        let mut num = nums[i as usize];
        num -= i;

        queue.push(num);
        queue.push(num);
        queue.pop();
        ans[i as usize] = queue.peek().unwrap().clone();
    }

    for i in (1..=n as usize - 1).rev() {
        ans[i - 1] = cmp::min(ans[i - 1], ans[i]);
    }

    for i in 0..n {
        writeln!(out, "{}", ans[i as usize] + i).unwrap();
    }
}
