use std::{collections::BinaryHeap, io};

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
    let n = input_integers()[0];

    let mut queue = BinaryHeap::new();
    let mut ans: i64 = 0;

    let nums = input_integers();

    for i in 0..n {
        let mut num = nums[i as usize];
        num -= i;

        queue.push(num);

        if !queue.is_empty() && queue.peek().unwrap() > &num {
            ans += (queue.peek().unwrap() - num) as i64;
            queue.pop();
            queue.push(num);
        }
    }

    println!("{}", ans);
}
