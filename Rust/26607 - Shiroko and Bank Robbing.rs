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

extern "C" {
    fn rand() -> u32;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k, x) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut items = Vec::new();
    let mut max_values = vec![vec![(0, 0); k + 1]; n + 1];
    let mut ret = 0;

    for _ in 0..n {
        let (a, _) = (scan.token::<usize>(), scan.token::<usize>());
        items.push((1, a, 0));
    }

    for _ in 0..1000 {
        items.iter_mut().for_each(|val| val.2 = unsafe { rand() });
        items.sort_by(|a, b| a.2.cmp(&b.2));

        for i in 0..n {
            for j in 0..=k {
                if i == 0 {
                    if items[i].0 <= j {
                        max_values[i][j] = (items[i].1, x - items[i].1);
                    }
                } else {
                    if items[i].0 <= j {
                        let val1 = max_values[i - 1][j].0 * max_values[i - 1][j].1;
                        let val2 = (max_values[i - 1][j - items[i].0].0 + items[i].1)
                            * (max_values[i - 1][j - items[i].0].1 + x - items[i].1);
    
                        max_values[i][j] = if val1 > val2 {
                            max_values[i - 1][j]
                        } else {
                            (
                                max_values[i - 1][j - items[i].0].0 + items[i].1,
                                max_values[i - 1][j - items[i].0].1 + x - items[i].1,
                            )
                        };
                    } else {
                        max_values[i][j] = max_values[i - 1][j];
                    }
                }
            }
        }

        ret = ret.max(max_values[n - 1][k].0 * max_values[n - 1][k].1);
    }

    writeln!(out, "{ret}").unwrap();
}
