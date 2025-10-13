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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, t) = (scan.token::<usize>(), scan.token::<i64>());
    let mut photographs = vec![(0, 0); n];
    let mut times_end = vec![0; n];

    for i in 0..n {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
        photographs[i] = (a, b);
        times_end[i] = b;
    }

    photographs.sort_unstable();
    times_end.sort_unstable();

    let mut times_start = photographs.iter().map(|&val| val.0).collect::<Vec<_>>();

    for i in 0..n {
        let mut time_remain = times_end[i] - t;
        let mut k = n - 1;

        for j in (0..n).rev() {
            if photographs[j].1 <= times_end[i] {
                while time_remain < photographs[k].0 {
                    time_remain = time_remain.min(times_start[k]);

                    if k == 0 {
                        break;
                    }

                    k -= 1;
                }

                if time_remain < photographs[j].0 {
                    writeln!(out, "no").unwrap();
                    return;
                }

                if time_remain < photographs[j].0 + t {
                    times_start[j] = times_start[j].min(time_remain - t);
                }

                time_remain -= t;
            }
        }
    }

    writeln!(out, "yes").unwrap();
}
