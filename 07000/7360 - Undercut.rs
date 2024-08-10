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

    let mut t = 0;

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        if t != 0 {
            writeln!(out).unwrap();
        }

        let mut cards_a = vec![0; n];
        let mut cards_b = vec![0; n];

        for i in 0..n {
            cards_a[i] = scan.token::<i64>();
        }

        for i in 0..n {
            cards_b[i] = scan.token::<i64>();
        }

        let mut point_a = 0;
        let mut point_b = 0;

        for i in 0..n {
            if cards_a[i] == cards_b[i] {
                continue;
            }

            if cards_a[i] - cards_b[i] == 1 {
                point_b += if cards_b[i] == 1 {
                    6
                } else {
                    cards_a[i] + cards_b[i]
                };
            } else if cards_b[i] - cards_a[i] == 1 {
                point_a += if cards_a[i] == 1 {
                    6
                } else {
                    cards_a[i] + cards_b[i]
                };
            } else if cards_a[i] > cards_b[i] {
                point_a += cards_a[i];
            } else if cards_b[i] > cards_a[i] {
                point_b += cards_b[i];
            }
        }

        writeln!(out, "A has {point_a} points. B has {point_b} points.").unwrap();

        t += 1;
    }
}
