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

    let k = scan.token::<i64>();

    for i in 1..=k {
        let n = scan.token::<usize>();
        let mut colors = vec![(0, 0, 0); n];

        for i in 0..n {
            colors[i] = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
        }

        let mut contrast_min = 0;
        let mut ret = Vec::new();

        for a in 0..n - 1 {
            for b in a + 1..n {
                let contrast = (colors[a].0 - colors[b].0).pow(2)
                    + (colors[a].1 - colors[b].1).pow(2)
                    + (colors[a].2 - colors[b].2).pow(2);

                if contrast > contrast_min {
                    contrast_min = contrast;
                    ret = vec![(a, b)];
                } else if contrast == contrast_min {
                    ret.push((a, b));
                }
            }
        }

        writeln!(out, "Data Set {i}:").unwrap();

        for (a, b) in ret {
            writeln!(out, "{} {}", a + 1, b + 1).unwrap();
        }
    }
}
