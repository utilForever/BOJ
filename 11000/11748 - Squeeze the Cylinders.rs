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
    let mut radius = vec![0.0; n];

    for i in 0..n {
        radius[i] = scan.token::<f64>();
    }

    let mut positions = vec![0.0; n];
    let mut ret = f64::MAX;

    positions[0] = radius[0];
    let mut positions_max = radius[0] * 2.0;

    for i in 1..n {
        let mut val = radius[i];

        for j in 0..i {
            val = val.max(positions[j] + (4.0 * radius[i] * radius[j]).sqrt());
        }

        positions[i] = val;
        positions_max = positions_max.max(positions[i] + radius[i]);
    }

    ret = ret.min(positions_max);

    writeln!(out, "{:.9}", ret).unwrap();
}
