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

    let (x, k) = (scan.token::<i64>(), scan.token::<u8>());
    let mut ret = i64::MAX;

    for i in 1..=18 {
        for j in 0..=9 {
            let mut num = String::new();

            for _ in 0..i {
                num.push((j as u8 + b'0') as char);
            }

            if k == 0 {
                let num = num.parse::<i64>().unwrap();

                if num >= x {
                    ret = ret.min(num);
                }
            } else {
                for k in 0..i {
                    for l in 0..=9 {
                        if l == j {
                            continue;
                        }

                        let mut num_temp = num.clone();
                        num_temp.replace_range(k..k + 1, &l.to_string());

                        let num = num_temp.parse::<i64>().unwrap();

                        if num >= x {
                            ret = ret.min(num);
                        }
                    }
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
