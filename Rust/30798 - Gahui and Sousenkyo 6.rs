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

    let (n, x) = (scan.token::<usize>(), scan.token::<i64>());
    let mut ret = vec![0; n];

    if n % 4 == 0 {
        for i in 0..n {
            ret[i] = (i as i64 + 4) * (1_i64 << 31);
        }

        ret[0] += x;
    } else {
        let (q, r) = (n / 4, n % 4);
        let basis = (1_i64 << 31) - 1 - x;

        for i in 0..4 * q {
            ret[i] = (i as i64 + 4) * (1_i64 << 31);
        }

        ret[0] += basis;

        let mut curr_val = (1_i64 << 31) - 1;
        let mut curr_bit = 30;

        for i in 0..r {
            if i == r - 1 {
                ret[4 * q + i] = curr_val;
            } else {
                curr_val -= 1_i64 << curr_bit;
                ret[4 * q + i] = 1_i64 << curr_bit;
                curr_bit -= 1;
            }
        }
    }

    ret.sort_by(|a, b| b.cmp(a));

    writeln!(out, "{n}").unwrap();

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
