use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn cross(stack: &Vec<(i64, i64)>, x: i64, y: i64) -> f64 {
    let t1 = (stack[y as usize].1 - stack[x as usize].1) as f64;
    let t2 = (stack[x as usize].0 - stack[y as usize].0) as f64;

    t1 / t2
}

fn insert(stack: &mut Vec<(i64, i64)>, size: &mut i64, x: i64, y: i64) {
    stack[*size as usize] = (x, y);

    while *size > 1 && cross(stack, *size - 2, *size - 1) > cross(stack, *size - 1, *size) {
        stack.swap(*size as usize - 1, *size as usize);
        *size -= 1;
    }

    *size += 1;
}

fn query(stack: &Vec<(i64, i64)>, size: i64, last: &mut i64, x: i64) -> i64 {
    while *last + 1 < size && cross(stack, *last, *last + 1) <= x as f64 {
        *last += 1;
    }

    x * stack[*last as usize].0 + stack[*last as usize].1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut n = scan.token::<usize>();
    let mut arr = vec![(0, 0); n];

    for i in 0..n {
        arr[i].0 = scan.token::<i64>();
        arr[i].1 = scan.token::<i64>();
    }

    arr.sort();
    arr.reverse();

    let mut new_arr = Vec::new();
    new_arr.push(arr[0]);

    for i in 1..n {
        if new_arr.last().unwrap().1 >= arr[i].1 {
            continue;
        }

        new_arr.push(arr[i]);
    }

    new_arr.sort();
    n = new_arr.len();

    let mut stack = vec![(0, 0); n + 1];
    let mut cost = vec![0; n + 1];
    let (mut size, mut last) = (0, 0);

    for i in 1..=n {
        insert(&mut stack, &mut size, new_arr[i - 1].1, cost[i - 1]);
        cost[i] = query(&stack, size, &mut last, new_arr[i - 1].0);
    }

    writeln!(out, "{}", cost[n]).unwrap();
}
