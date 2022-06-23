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

fn calculate_d(a: i64, b: i64, c: i64) -> i64 {
    b * b - 4 * a * c
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (a, mut b, mut c, mut d) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let mut x = 0;

        for i in 0..=2_000_000 {
            if a * i * i * i + b * i * i + c * i + d == 0 {
                x = i;
                break;
            }

            if -a * i * i * i + b * i * i - c * i + d == 0 {
                x = -i;
                break;
            }
        }

        d = a * x * x + b * x + c;
        c = a * x + b;
        b = a;

        if calculate_d(b, c, d) < 0 {
            writeln!(out, "{:.10}", x).unwrap();
        } else {
            let mut ret = vec![
                x as f64,
                (-c as f64 + (calculate_d(b, c, d) as f64).sqrt()) / (2.0 * b as f64),
                (-c as f64 - (calculate_d(b, c, d) as f64).sqrt()) / (2.0 * b as f64),
            ];
            ret.sort_by(|a, b| a.partial_cmp(b).unwrap());
            ret.dedup();

            for val in ret.iter() {
                write!(out, "{:.10} ", val).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
