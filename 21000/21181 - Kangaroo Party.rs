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
    let mut locations = vec![0; n];

    for i in 0..n {
        locations[i] = scan.token::<i64>();
    }

    let mut ret = i64::MAX;

    for i in 0..n {
        for j in i + 1..n {
            let mut energy = 0;

            for k in 0..n {
                if k == i || k == j {
                    continue;
                }

                let dist1 = (locations[i] - locations[k]).abs();
                let dist2 = (locations[j] - locations[k]).abs();

                energy += (dist1 * dist1).min(dist2 * dist2);
            }

            ret = ret.min(energy);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
