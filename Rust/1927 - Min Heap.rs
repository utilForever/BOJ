use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io};

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

    let n = input_integers()[0] as usize;

    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(n);

    for _ in 0..n {
        let num = input_integers()[0];

        if num == 0 {
            if heap.is_empty() {
                writeln!(out, "0").unwrap();
            } else {
                writeln!(out, "{}", heap.pop().unwrap().0).unwrap();
            }
        } else {
            heap.push(Reverse(num));
        }
    }
}
