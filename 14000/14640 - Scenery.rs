use io::Write;
use std::{cmp, io, str};

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

    let (n, t) = (scan.token::<usize>(), scan.token::<i64>());
    let mut photographs = vec![(0, 0); n];
    let mut end_times = vec![0; n];

    for i in 0..n {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
        photographs[i] = (a, b);
        end_times[i] = b;
    }

    photographs.sort();
    end_times.sort();

    let mut start_times = photographs.iter().map(|&val| val.0).collect::<Vec<_>>();

    for i in 0..n {
        let mut remain_time = end_times[i] - t;
        let mut k = n as i64 - 1;

        for j in (0..n).rev() {
            if photographs[j].1 <= end_times[i] {
                while k >= 0 && remain_time < photographs[k as usize].0 {
                    remain_time = cmp::min(remain_time, start_times[k as usize]);
                    k -= 1;
                }

                if remain_time < photographs[j].0 {
                    writeln!(out, "no").unwrap();
                    return;
                }

                if remain_time < photographs[j].0 + t {
                    start_times[j] = cmp::min(start_times[j], remain_time - t);
                }

                remain_time -= t;
            }
        }
    }

    writeln!(out, "yes").unwrap();
}
