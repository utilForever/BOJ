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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut a = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<i64>();
    }

    let mut stack = Vec::new();
    let mut ret = vec![0; n];

    for i in (0..n).rev() {
        while !stack.is_empty() && a[*stack.last().unwrap()] <= a[i] {
            stack.pop();
        }

        ret[i] = if stack.is_empty() {
            -1
        } else {
            *stack.last().unwrap() as i64
        };

        stack.push(i);
    }

    for val in ret {
        write!(out, "{} ", val + 1).unwrap();
    }

    writeln!(out).unwrap();
}
