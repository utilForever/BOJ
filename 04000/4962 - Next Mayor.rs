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

    loop {
        let (n, p) = (scan.token::<usize>(), scan.token::<i64>());

        if n == 0 && p == 0 {
            break;
        }

        let mut stones = vec![0; n];
        let mut idx = 0;
        let mut bowl = p;
        let mut ret;

        loop {
            if bowl > 0 {
                stones[idx] += 1;
                bowl -= 1;
                idx = (idx + 1) % n;
            } else {
                ret = (idx + n - 1) % n;
                let mut check = true;

                for i in 0..n {
                    if i == ret {
                        continue;
                    }

                    if stones[i] != 0 {
                        check = false;
                        break;
                    }
                }

                if check {
                    break;
                } else {
                    bowl = stones[idx];
                    stones[idx] = 0;
                    idx = (idx + 1) % n;
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
