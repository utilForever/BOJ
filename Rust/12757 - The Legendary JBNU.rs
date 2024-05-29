use io::Write;
use std::{collections::BTreeMap, io, str};

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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut map = BTreeMap::new();

    for _ in 0..n {
        let (key, value) = (scan.token::<i64>(), scan.token::<i64>());
        map.insert(key, value);
    }

    for _ in 0..m {
        let command = scan.token::<i64>();

        if command == 1 {
            let (key, value) = (scan.token::<i64>(), scan.token::<i64>());
            map.insert(key, value);
        } else if command == 2 {
            let (key, value) = (scan.token::<i64>(), scan.token::<i64>());
            let left = map.range(key - k..=key);
            let mut right = map.range(key..=key + k);

            let left_last = left.last();
            let right_first = right.next();

            match (left_last, right_first) {
                (Some((k1, _)), Some((k2, _))) => {
                    if (key - k1).abs() == (k2 - key).abs() {
                        if key == *k1 {
                            map.entry(key).and_modify(|v| *v = value);
                        }
                    } else if (key - k1).abs() < (k2 - key).abs() {
                        map.entry(*k1).and_modify(|v| *v = value);
                    } else {
                        map.entry(*k2).and_modify(|v| *v = value);
                    }
                }
                (Some((k1, _)), None) => {
                    map.entry(*k1).and_modify(|v| *v = value);
                }
                (None, Some((k2, _))) => {
                    map.entry(*k2).and_modify(|v| *v = value);
                }
                _ => (),
            }
        } else {
            let key = scan.token::<i64>();
            let left = map.range(key - k..=key);
            let mut right = map.range(key..=key + k);

            let left_last = left.last();
            let right_first = right.next();

            match (left_last, right_first) {
                (Some((k1, v1)), Some((k2, v2))) => {
                    if (key - k1).abs() == (k2 - key).abs() {
                        if key == *k1 {
                            writeln!(out, "{v1}").unwrap();
                        } else {
                            writeln!(out, "?").unwrap();
                        }
                    } else if (key - k1).abs() < (k2 - key).abs() {
                        writeln!(out, "{v1}").unwrap();
                    } else {
                        writeln!(out, "{v2}").unwrap();
                    }
                }
                (Some((_, v1)), None) => {
                    writeln!(out, "{v1}").unwrap();
                }
                (None, Some((_, v2))) => {
                    writeln!(out, "{v2}").unwrap();
                }
                _ => {
                    writeln!(out, "-1").unwrap();
                }
            }
        }
    }
}
