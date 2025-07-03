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

    let (w, h) = (scan.token::<i64>(), scan.token::<i64>());
    let n = scan.token::<usize>();
    let mut boundaries = Vec::new();

    for i in 0..n {
        let mut bounds = [-1, -1];

        for j in 0..2 {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());

            if x != 0 && x != w && y != 0 && y != h {
                continue;
            }

            let boundary = if y == 0 {
                x
            } else if x == w {
                w + y
            } else if y == h {
                w + h + (w - x)
            } else {
                w + h + w + (h - y)
            };

            bounds[j] = boundary;
        }

        if bounds[0] == -1 || bounds[1] == -1 {
            continue;
        }

        boundaries.push((bounds[0], i));
        boundaries.push((bounds[1], i));
    }

    boundaries.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut stack = Vec::new();

    for (_, color) in boundaries {
        if stack.last() == Some(&color) {
            stack.pop();
        } else {
            stack.push(color);
        }
    }

    writeln!(out, "{}", if stack.is_empty() { "Y" } else { "N" }).unwrap();
}
