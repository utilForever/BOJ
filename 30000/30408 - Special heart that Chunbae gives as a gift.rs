use io::Write;
use std::{collections::VecDeque, io, str};

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

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());

    if n == m {
        writeln!(out, "YES").unwrap();
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back(n);

    while !queue.is_empty() {
        let weight = queue.pop_front().unwrap();

        if weight < m {
            continue;
        }

        if weight % 2 == 0 {
            let a = weight / 2;

            if a == m {
                writeln!(out, "YES").unwrap();
                return;
            }

            queue.push_back(a);
        } else {
            let a = (weight - 1) / 2;
            let b = (weight - 1) / 2 + 1;

            if a == m || b == m {
                writeln!(out, "YES").unwrap();
                return;
            }

            if a % 2 == 0 {
                queue.push_back(b);
            } else {
                queue.push_back(a);
            }
        }
    }

    writeln!(out, "NO").unwrap();
}
