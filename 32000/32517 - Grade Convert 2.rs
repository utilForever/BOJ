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

    let n = scan.token::<usize>();
    let mut b = vec![0; n];

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    let mut a = vec![0; n];
    let mut a_sum = 0;

    a[0] = b[0];
    a_sum += a[0];

    if a[0] < 1 || a[0] > 1_000_000_000 {
        writeln!(out, "-1").unwrap();
        return;
    }

    for i in 1..n {
        let a_avg = a_sum as f64 / i as f64;

        a[i] = if b[i] as f64 + 1.0 >= a_avg {
            b[i]
        } else {
            b[i] + 1
        };

        if a[i] < 1 || a[i] > 1_000_000_000 {
            writeln!(out, "-1").unwrap();
            return;
        }

        a_sum += a[i];
    }

    let mut a_sum = a[0];

    if a[0] != b[0] {
        writeln!(out, "-1").unwrap();
        return;
    }

    for i in 1..n {
        let a_avg = a_sum as f64 / i as f64;

        if (a[i] as f64) < a_avg && b[i] != a[i] - 1 {
            writeln!(out, "-1").unwrap();
            return;
        } else if (a[i] as f64) >= a_avg && b[i] != a[i] {
            writeln!(out, "-1").unwrap();
            return;
        }

        a_sum += a[i];
    }

    for val in a {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
