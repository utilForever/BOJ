use io::Write;
use std::{cmp::Ordering, collections::BinaryHeap, io};

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

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default, Hash)]
pub struct Absolute<T>(pub T);

impl PartialOrd for Absolute<i32> {
    #[inline]
    fn partial_cmp(&self, other: &Absolute<i32>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        if other.0.abs() == self.0.abs() {
            return other.0 < self.0;
        }

        other.0.abs() < self.0.abs()
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        if other.0.abs() == self.0.abs() {
            return other.0 <= self.0;
        }

        other.0.abs() <= self.0.abs()
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        if other.0.abs() == self.0.abs() {
            return other.0 > self.0;
        }

        other.0.abs() > self.0.abs()
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        if other.0.abs() == self.0.abs() {
            return other.0 >= self.0;
        }

        other.0.abs() >= self.0.abs()
    }
}

impl Ord for Absolute<i32> {
    #[inline]
    fn cmp(&self, other: &Absolute<i32>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = input_integers()[0] as usize;

    let mut heap: BinaryHeap<Absolute<i32>> = BinaryHeap::with_capacity(n);

    for _ in 0..n {
        let num = input_integers()[0];

        if num == 0 {
            if heap.is_empty() {
                writeln!(out, "0").unwrap();
            } else {
                writeln!(out, "{}", heap.pop().unwrap().0).unwrap();
            }
        } else {
            heap.push(Absolute(num));
        }
    }
}
