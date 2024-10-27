use io::Write;
use std::{collections::BinaryHeap, io, str};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
    let mut x_decomposed = BinaryHeap::new();
    let mut y_decomposed = BinaryHeap::new();

    let mut val = x;

    while val > 0 {
        for i in (1i64..=60).rev() {
            while val >= (1 << i) - 1 {
                x_decomposed.push((1 << i) - 1);
                val -= (1 << i) - 1;
            }
        }
    }

    let mut val = y;

    while val > 0 {
        for i in (1i64..=60).rev() {
            while val >= (1 << i) - 1 {
                y_decomposed.push((1 << i) - 1);
                val -= (1 << i) - 1;
            }
        }
    }

    // writeln!(out, "Before X: {:?}", x_decomposed).unwrap();
    // writeln!(out, "Before Y: {:?}", y_decomposed).unwrap();

    while x_decomposed.len() + 1 < y_decomposed.len() {
        if x_decomposed.is_empty() || *x_decomposed.peek().unwrap() == 1 {
            writeln!(out, "impossible").unwrap();
            return;
        }

        let val = x_decomposed.pop().unwrap();

        x_decomposed.push(val / 2);
        x_decomposed.push(val / 2);
        x_decomposed.push(1);
    }

    while y_decomposed.len() + 1 < x_decomposed.len() {
        if y_decomposed.is_empty() || *y_decomposed.peek().unwrap() == 1 {
            writeln!(out, "impossible").unwrap();
            return;
        }

        let val = y_decomposed.pop().unwrap();

        y_decomposed.push(val / 2);
        y_decomposed.push(val / 2);
        y_decomposed.push(1);
    }

    // writeln!(out, "After X: {:?}", x_decomposed).unwrap();
    // writeln!(out, "After Y: {:?}", y_decomposed).unwrap();

    if y_decomposed.len() > x_decomposed.len() {
        let val_y: i64 = y_decomposed.pop().unwrap();

        for _ in 0..val_y.count_ones() {
            write!(out, "U").unwrap();
        }

        while !x_decomposed.is_empty() {
            let val_x: i64 = x_decomposed.pop().unwrap();
            let val_y: i64 = y_decomposed.pop().unwrap();

            for _ in 0..val_x.count_ones() {
                write!(out, "R").unwrap();
            }

            for _ in 0..val_y.count_ones() {
                write!(out, "U").unwrap();
            }
        }
    } else if x_decomposed.len() > y_decomposed.len() {
        let val_x: i64 = x_decomposed.pop().unwrap();

        for _ in 0..val_x.count_ones() {
            write!(out, "R").unwrap();
        }

        while !x_decomposed.is_empty() {
            let val_x: i64 = x_decomposed.pop().unwrap();
            let val_y: i64 = y_decomposed.pop().unwrap();

            for _ in 0..val_y.count_ones() {
                write!(out, "U").unwrap();
            }

            for _ in 0..val_x.count_ones() {
                write!(out, "R").unwrap();
            }
        }
    } else {
        while !x_decomposed.is_empty() {
            let val_x: i64 = x_decomposed.pop().unwrap();
            let val_y: i64 = y_decomposed.pop().unwrap();

            for _ in 0..val_x.count_ones() {
                write!(out, "R").unwrap();
            }

            for _ in 0..val_y.count_ones() {
                write!(out, "U").unwrap();
            }
        }
    }
}
