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

    let mut rectangles = Vec::new();

    for i in 1..=199 {
        for j in i + 1..=200 {
            rectangles.push((i, j));
        }
    }

    rectangles.sort_by(|a, b| {
        let diagonal_a = a.0 * a.0 + a.1 * a.1;
        let diagonal_b = b.0 * b.0 + b.1 * b.1;

        if diagonal_a == diagonal_b {
            a.0.cmp(&b.0)
        } else {
            diagonal_a.cmp(&diagonal_b)
        }
    });

    loop {
        let (h, w) = (scan.token::<i64>(), scan.token::<i64>());

        if h == 0 && w == 0 {
            break;
        }

        let pos = rectangles.iter().position(|&x| x == (h, w)).unwrap() + 1;

        writeln!(out, "{} {}", rectangles[pos].0, rectangles[pos].1).unwrap();
    }
}
