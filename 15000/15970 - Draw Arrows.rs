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
    let mut points = vec![Vec::new(); n + 1];

    for _ in 0..n {
        let (pos, color) = (scan.token::<i64>(), scan.token::<usize>());
        points[color].push(pos);
    }

    for i in 1..=n {
        points[i].sort();
    }

    let mut ret = 0;

    for i in 1..=n {
        let len = points[i].len();

        if len >= 3 {
            for j in 1..len - 1 {
                ret += (points[i][j] - points[i][j - 1]).min(points[i][j + 1] - points[i][j]);
            }
        }

        if len >= 2 {
            ret += points[i][1] - points[i][0];
            ret += points[i][len - 1] - points[i][len - 2];
        }
    }

    writeln!(out, "{ret}").unwrap();
}
