use io::Write;
use std::{io, str, vec};

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

    let (n, x) = (scan.token::<usize>(), scan.token::<usize>());
    let mut visits = vec![0; n];
    let mut prefix_sum = vec![0; n + 1];

    for i in 0..n {
        visits[i] = scan.token::<i64>();
    }

    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + visits[i];
    }

    let mut ret = 0;
    let mut cnt = 0;

    for i in x..=n {
        let val = prefix_sum[i] - prefix_sum[i - x];

        if val > ret {
            ret = val;
            cnt = 1;
        } else if val == ret {
            cnt += 1;
        }
    }

    if ret == 0 {
        writeln!(out, "SAD").unwrap();
    } else {
        writeln!(out, "{ret}").unwrap();
        writeln!(out, "{cnt}").unwrap();
    }
}
