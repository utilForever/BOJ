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

fn cross(stack: &Vec<(i64, i64)>, x: usize, y: usize) -> f64 {
    let t1 = stack[y].1 - stack[x].1;
    let t2 = stack[x].0 - stack[y].0;

    t1 as f64 / t2 as f64
}

fn insert(stack: &mut Vec<(i64, i64)>, size: &mut usize, x: i64, y: i64) {
    stack[*size] = (x, y);

    while *size > 1 && cross(stack, *size - 2, *size - 1) > cross(stack, *size - 1, *size) {
        stack[*size - 1] = stack[*size];
        *size -= 1;
    }

    *size += 1;
}

fn query(stack: &Vec<(i64, i64)>, size: &usize, last: &mut usize, x: i64) -> i64 {
    while *last + 1 < *size && cross(stack, *last, *last + 1) <= x as f64 {
        *last += 1;
    }

    x * stack[*last].0 as i64 + stack[*last].1 as i64
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];

    for i in 1..=n {
        a[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        b[i] = scan.token::<i64>();
    }

    let mut stack = vec![(0, 0); n + 1];
    let mut size = 0;

    insert(&mut stack, &mut size, b[1], 0);

    let mut cost = vec![0; n + 1];
    let mut last = 0;

    for i in 2..=n {
        cost[i] = query(&stack, &size, &mut last, a[i]);
        insert(&mut stack, &mut size, b[i], cost[i]);
    }

    writeln!(out, "{}", cost[n]).unwrap();
}
