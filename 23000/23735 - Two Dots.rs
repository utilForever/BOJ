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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (l, n) = (scan.token::<i64>(), scan.token::<usize>());
    let mut boundaries = Vec::new();
    let mut boundaries_cnt = vec![0; n / 2 + 1];

    for _ in 0..n {
        let (x, y, c) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
        );

        if x != 0 && x != l && y != 0 && y != l {
            continue;
        }

        let boundary = if y == 0 {
            x
        } else if x == l {
            l + y
        } else if y == l {
            2 * l + (l - x)
        } else {
            3 * l + (l - y)
        };

        boundaries.push((boundary, c));
        boundaries_cnt[c] += 1;
    }

    boundaries.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut stack = Vec::new();
    let mut ret = true;

    for (_, color) in boundaries {
        if boundaries_cnt[color] != 2 {
            continue;
        }

        if stack.last() == Some(&color) {
            stack.pop();
        } else if stack.iter().any(|&c| c == color) {
            ret = false;
            break;
        } else {
            stack.push(color);
        }
    }

    writeln!(out, "{}%", if ret && stack.is_empty() { "1" } else { "0" }).unwrap();
}
