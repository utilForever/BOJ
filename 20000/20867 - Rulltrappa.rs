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

    let (m, s, g) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (a, b) = (scan.token::<f64>(), scan.token::<f64>());
    let (l, r) = (scan.token::<i64>(), scan.token::<i64>());

    let time_l = if m % g == 0 { m / g } else { m / g + 1 };
    let time_r = if m % s == 0 { m / s } else { m / s + 1 };
    let wait_l = l as f64 / a;
    let wait_r = r as f64 / b;

    writeln!(
        out,
        "{}",
        if time_l as f64 + wait_l > time_r as f64 + wait_r {
            "latmask"
        } else {
            "friskus"
        }
    )
    .unwrap();
}
