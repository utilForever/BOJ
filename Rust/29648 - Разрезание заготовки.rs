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

    let (a, b, s) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );

    // NOTE: (l - a) * (l - b) = s
    // l^2 - (a + b) * l + a * b - s = 0
    // l = ((a + b) + sqrt((a + b)^2 - 4 * (a * b - s))) / 2
    let ret = (((a + b) + ((a + b) * (a + b) - 4.0 * (a * b - s)).sqrt()) / 2.0).round() as i64;

    writeln!(
        out,
        "{}",
        if (ret - a as i64) * (ret - b as i64) == s as i64 {
            ret
        } else {
            -1
        }
    )
    .unwrap();
}
