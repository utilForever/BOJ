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

    let (a, b, c, d) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let mut val_max = (a / c) + (b / d);
    let mut num_rotation = 0;

    if (c / d) + (a / b) > val_max {
        val_max = (c / d) + (a / b);
        num_rotation = 1;
    }

    if (d / b) + (c / a) > val_max {
        val_max = (d / b) + (c / a);
        num_rotation = 2;
    }

    if (b / a) + (d / c) > val_max {
        num_rotation = 3;
    }

    writeln!(out, "{num_rotation}").unwrap();
}
