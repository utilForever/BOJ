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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut positions = vec![(0, 0); n];

    for i in 0..n {
        positions[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = i64::MAX;

    if k == 1 {
        for i in 0..n {
            let mut dist_max = 0;

            for j in 0..n {
                if i == j {
                    continue;
                }

                let dist = (positions[i].0 - positions[j].0).abs()
                    + (positions[i].1 - positions[j].1).abs();
                dist_max = dist_max.max(dist);
            }

            ret = ret.min(dist_max);
        }
    } else if k == 2 {
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let mut dist_max = 0;

                for k in 0..n {
                    if i == k || j == k {
                        continue;
                    }

                    let dist1 = (positions[k].0 - positions[i].0).abs()
                        + (positions[k].1 - positions[i].1).abs();
                    let dist2 = (positions[k].0 - positions[j].0).abs()
                        + (positions[k].1 - positions[j].1).abs();

                    dist_max = dist_max.max(dist1.min(dist2));
                }

                ret = ret.min(dist_max);
            }
        }
    } else {
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                for k in 0..n {
                    if i == k || j == k {
                        continue;
                    }

                    let mut dist_max = 0;

                    for l in 0..n {
                        if i == l || j == l || k == l {
                            continue;
                        }

                        let dist1 = (positions[l].0 - positions[i].0).abs()
                            + (positions[l].1 - positions[i].1).abs();
                        let dist2 = (positions[l].0 - positions[j].0).abs()
                            + (positions[l].1 - positions[j].1).abs();
                        let dist3 = (positions[l].0 - positions[k].0).abs()
                            + (positions[l].1 - positions[k].1).abs();

                        dist_max = dist_max.max(dist1.min(dist2.min(dist3)));
                    }

                    ret = ret.min(dist_max);
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
