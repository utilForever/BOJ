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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, k) = (scan.token::<i64>(), scan.token::<i64>());
        let mut ret = [false; 7];

        let mut mark = |a: i64, b: i64| {
            let (i, j) = if a < b { (a, b) } else { (b, a) };
            let idx = match (i, j) {
                (1, 2) => 1,
                (1, 3) => 2,
                (1, 4) => 3,
                (2, 3) => 4,
                (2, 4) => 5,
                (3, 4) => 6,
                _ => unreachable!(),
            };

            ret[idx] = true;
        };

        if k == 1 {
            let pos = ((n - 1) % 4) as i64 + 1;
            let seats = [1, 2, 3, 4];
            let others = seats
                .iter()
                .filter(|&&x| x != pos)
                .cloned()
                .collect::<Vec<_>>();

            mark(others[0], others[1]);
            mark(others[0], others[2]);
            mark(others[1], others[2]);
        } else {
            let (r, p) = (n % (k + 1), (n / (k + 1)) % 2);

            if r == 1 {
                mark(2, 4);
            } else {
                mark(1, 3);
            }

            let g = if p == 0 {
                if r <= 2 {
                    r + 2
                } else {
                    1
                }
            } else {
                if r <= 1 {
                    1
                } else {
                    2
                }
            };

            let left = if g == 1 { 4 } else { g - 1 };
            let right = if g == 4 { 1 } else { g + 1 };

            mark(left, g);
            mark(g, right);
        }

        if ret[1] {
            write!(out, "(1,2) ").unwrap();
        }

        if ret[2] {
            write!(out, "(1,3) ").unwrap();
        }

        if ret[3] {
            write!(out, "(1,4) ").unwrap();
        }

        if ret[4] {
            write!(out, "(2,3) ").unwrap();
        }

        if ret[5] {
            write!(out, "(2,4) ").unwrap();
        }

        if ret[6] {
            write!(out, "(3,4) ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
