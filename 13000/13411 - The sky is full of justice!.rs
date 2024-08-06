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
    let mut missiles = vec![(0, 0, 0, 0); n];

    for i in 0..n {
        missiles[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            i,
        );
    }

    missiles.sort_by(|a, b| {
        let dist_a = a.0 * a.0 + a.1 * a.1;
        let dist_b = b.0 * b.0 + b.1 * b.1;
        let time_a = (dist_a as f64).sqrt() / a.2 as f64;
        let time_b = (dist_b as f64).sqrt() / b.2 as f64;

        if time_a < time_b {
            return std::cmp::Ordering::Less;
        } else if time_a > time_b {
            return std::cmp::Ordering::Greater;
        } else {
            a.3.cmp(&b.3)
        }
    });

    for i in 0..n {
        writeln!(out, "{}", missiles[i].3 + 1).unwrap();
    }
}
